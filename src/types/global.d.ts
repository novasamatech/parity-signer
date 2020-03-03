export {};

/*~ If the app has properties exposed on a global variable,
 *~ place them here.
 *~ You should also place types (interfaces and type alias) here.
 */
/* eslint-disable prettier/prettier */
declare global {
    namespace NodeJS {
        interface Global {
            inTest: boolean;
        }
    }
    // declare webpack modules
    declare module '*.png'
}

