//
//  MLog+Generate.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/01/2024.
//

import Foundation
@testable import PolkadotVault

extension MLog {
    static func generate(
        log: [History] = [History.generate()]
    ) -> MLog {
        MLog(
            log: log
        )
    }
}

extension History {
    static func generate(
        order: UInt32 = 0,
        timestamp: String = "2021-01-01T00:00:00Z",
        events: [Event] = [Event.generate()]
    ) -> History {
        History(
            order: order,
            timestamp: timestamp,
            events: events
        )
    }
}

extension Event {
    static func generate() -> Event {
        .identitiesWiped
    }
}

extension MLogDetails {
    static func generate(
        timestamp: String = "2021-01-01T00:00:00Z",
        events: [MEventMaybeDecoded] = [MEventMaybeDecoded.generate()]
    ) -> MLogDetails {
        MLogDetails(
            timestamp: timestamp,
            events: events
        )
    }
}

extension MEventMaybeDecoded {
    static func generate(
        event: Event = Event.generate(),
        signedBy: MAddressCard? = .generate(),
        decoded: TransactionCardSet? = .generate(),
        verifierDetails: MVerifierDetails? = .generate()
    ) -> MEventMaybeDecoded {
        MEventMaybeDecoded(
            event: event,
            signedBy: signedBy,
            decoded: decoded,
            verifierDetails: verifierDetails
        )
    }
}
