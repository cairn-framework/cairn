// Minimal PNG decoder (8-bit, non-interlaced, colour types 0/2/4/6) using the
// built-in zlib. Enough to turn Chrome's screenshot PNGs into raw pixels so the
// visual eval can derive a luminance-distribution signal straight from the
// captured image (the "screenshots through visual evals" requirement).

import zlib from "node:zlib";

const SIGNATURE = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);

function paeth(a, b, c) {
  const p = a + b - c;
  const pa = Math.abs(p - a);
  const pb = Math.abs(p - b);
  const pc = Math.abs(p - c);
  if (pa <= pb && pa <= pc) return a;
  if (pb <= pc) return b;
  return c;
}

/** Decode a PNG buffer to `{ width, height, channels, data }` (raw bytes). */
export function decodePng(buf) {
  if (!buf.subarray(0, 8).equals(SIGNATURE)) {
    throw new Error("not a PNG (bad signature)");
  }
  let pos = 8;
  let width = 0;
  let height = 0;
  let bitDepth = 0;
  let colorType = 0;
  let interlace = 0;
  const idat = [];
  while (pos + 8 <= buf.length) {
    const len = buf.readUInt32BE(pos);
    const type = buf.toString("ascii", pos + 4, pos + 8);
    const dataStart = pos + 8;
    const data = buf.subarray(dataStart, dataStart + len);
    pos = dataStart + len + 4; // skip chunk data + CRC
    if (type === "IHDR") {
      width = data.readUInt32BE(0);
      height = data.readUInt32BE(4);
      bitDepth = data[8];
      colorType = data[9];
      interlace = data[12];
    } else if (type === "IDAT") {
      idat.push(data);
    } else if (type === "IEND") {
      break;
    }
  }
  if (bitDepth !== 8) throw new Error(`unsupported PNG bit depth ${bitDepth}`);
  if (interlace !== 0) throw new Error("interlaced PNG not supported");
  const channels = { 0: 1, 2: 3, 4: 2, 6: 4 }[colorType];
  if (!channels) throw new Error(`unsupported PNG colour type ${colorType}`);

  const raw = zlib.inflateSync(Buffer.concat(idat));
  const stride = width * channels;
  const out = Buffer.alloc(height * stride);
  let rp = 0;
  for (let y = 0; y < height; y++) {
    const filter = raw[rp++];
    const rowStart = y * stride;
    const prevStart = rowStart - stride;
    for (let x = 0; x < stride; x++) {
      const rawByte = raw[rp++];
      const a = x >= channels ? out[rowStart + x - channels] : 0;
      const b = y > 0 ? out[prevStart + x] : 0;
      const c = x >= channels && y > 0 ? out[prevStart + x - channels] : 0;
      let val;
      switch (filter) {
        case 0:
          val = rawByte;
          break;
        case 1:
          val = rawByte + a;
          break;
        case 2:
          val = rawByte + b;
          break;
        case 3:
          val = rawByte + ((a + b) >> 1);
          break;
        case 4:
          val = rawByte + paeth(a, b, c);
          break;
        default:
          throw new Error(`bad PNG filter type ${filter}`);
      }
      out[rowStart + x] = val & 0xff;
    }
  }
  return { width, height, channels, data: out };
}

/**
 * Sample luminance across an image. Returns mean/std plus the count of distinct
 * coarse-quantised colours (a calm-vs-cluttered proxy). `std` near zero means a
 * blank/near-uniform render.
 */
export function imageStats(img, step = 4) {
  const { width, height, channels, data } = img;
  let n = 0;
  let sum = 0;
  let sumSq = 0;
  const colors = new Set();
  for (let y = 0; y < height; y += step) {
    for (let x = 0; x < width; x += step) {
      const i = (y * width + x) * channels;
      const r = data[i];
      const g = channels >= 3 ? data[i + 1] : r;
      const b = channels >= 3 ? data[i + 2] : r;
      const lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
      sum += lum;
      sumSq += lum * lum;
      n += 1;
      // quantise to 5 bits per channel for a stable distinct-colour count
      colors.add(((r >> 3) << 10) | ((g >> 3) << 5) | (b >> 3));
    }
  }
  const mean = n ? sum / n : 0;
  const variance = n ? sumSq / n - mean * mean : 0;
  return {
    mean,
    std: Math.sqrt(Math.max(0, variance)),
    samples: n,
    distinctColors: colors.size,
  };
}
