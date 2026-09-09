export class GaiaError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "GaiaError";
  }
}

export class InvalidArgument extends GaiaError {
  constructor(message: string) {
    super(message);
    this.name = "InvalidArgument";
  }
}

export class NotImplementedCapability extends GaiaError {
  constructor(message: string) {
    super(message);
    this.name = "NotImplementedCapability";
  }
}
