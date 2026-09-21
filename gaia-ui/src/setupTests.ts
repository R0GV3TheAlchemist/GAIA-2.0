// jsdom does not implement scrollIntoView — stub it so components that call
// element.scrollIntoView() don't throw in the test environment.
window.HTMLElement.prototype.scrollIntoView = function () {};
