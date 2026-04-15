export interface SourcePosition {
  line: number;
  column: number;
}

export class CairnError extends Error {
  readonly source?: string;
  readonly position?: SourcePosition;

  constructor(message: string, options: { source?: string; position?: SourcePosition } = {}) {
    super(message);
    this.name = new.target.name;
    this.source = options.source;
    this.position = options.position;
  }

  format(): string {
    const location = this.source && this.position ? `${this.source}:${this.position.line}:${this.position.column}` : undefined;
    return location ? `${location}: ${this.message}` : this.message;
  }
}

export class ParseError extends CairnError {}

export class StructuralError extends CairnError {}
