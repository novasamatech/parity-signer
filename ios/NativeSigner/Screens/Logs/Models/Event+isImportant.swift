//
//  Event+isWarning.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import SwiftUI

extension Event {
    var isWarning: Bool {
        switch self {
        case .deviceWasOnline,
             .secretWasExported,
             .resetDangerRecord,
             .transactionSignError,
             .typesRemoved,
             .warning,
             .wrongPassword,
             .messageSignError:
            return true
        default:
            return false
        }
    }
}
