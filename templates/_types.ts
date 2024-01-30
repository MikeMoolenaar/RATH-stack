import * as Htmx from '../static/node_modules/htmx.org'

declare global {
    var htmx: typeof Htmx;

    interface Window {
        htmx: typeof Htmx;
    }
}
