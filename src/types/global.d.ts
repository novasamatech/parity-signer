// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

/* eslint-disable @typescript-eslint/no-unused-vars */
import { ScanTestRequest } from 'e2e/mockScanRequests';
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
            scanRequest?: ScanTestRequest;
        }
    }
    // declare webpack modules
    declare module '*.png'
}

