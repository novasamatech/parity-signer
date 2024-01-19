//
//  LogEntryRenderable.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 07/12/2022.
//

import Foundation

struct LogEntryRenderable: Equatable, Hashable, Identifiable {
    enum EntryType: Equatable {
        case basic
        case bottomDetails
        case fullDetails
    }

    let id = UUID()
    let title: String
    let displayValue: String?
    let additionalValue: String?
    let isWarning: Bool
    let type: EntryType
    let dateHeader: String?
    let timestamp: String
    let navigationDetails: UInt32
}

final class LogEntryRenderableBuilder {
    func build(_ logs: MLog) -> [LogEntryRenderable] {
        var lastDate: String?
        var shouldIncludeDate = false
        var result: [LogEntryRenderable] = []
        // Used to retrieve data index for navigation...
        logs.log.forEach { historyItem in
            let timestamp = historyItem.timestamp
            historyItem.events.forEach {
                let dayString = DateFormatter.monthDay(timestamp)
                if dayString != lastDate {
                    lastDate = dayString
                    shouldIncludeDate = true
                } else {
                    shouldIncludeDate = false
                }
                result.append(
                    LogEntryRenderable(
                        title: $0.eventTitle,
                        displayValue: $0.displayValue,
                        additionalValue: $0.additionalValue,
                        isWarning: $0.isWarning,
                        type: $0.entryType,
                        dateHeader: shouldIncludeDate ? lastDate : nil,
                        timestamp: DateFormatter.hourMinutes(timestamp),
                        navigationDetails: UInt32(logs.log.reversed().firstIndex(of: historyItem) ?? 0)
                    )
                )
            }
        }
        return result
    }
}
