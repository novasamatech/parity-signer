// Prevent auto importing console when console logging...

declare module 'console' {
    export = typeof import("console");
}