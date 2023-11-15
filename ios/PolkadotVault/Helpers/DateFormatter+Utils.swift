//
//  DateFormatter+Utils.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import Foundation

extension DateFormatter {
    static var logDateFormatter = {
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "yyyy-MM-dd HH:mm:ss.SSSSSS"
        dateFormatter.locale = Locale(identifier: "en-US")
        return dateFormatter
    }()

    static var monthDayFormatter = {
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "MMM dd"
        dateFormatter.locale = Locale(identifier: "en-US")
        return dateFormatter
    }()

    static var hourMinutesFormatter = {
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = "HH:mm"
        dateFormatter.locale = Locale(identifier: "en-US")
        return dateFormatter
    }()

    static func parseLogTime(_ rustTimestamp: String) -> Date {
        logDateFormatter.date(from: rustTimestamp) ?? .now
    }

    static func hourMinutes(_ rustTimestamp: String) -> String {
        hourMinutesFormatter.string(from: parseLogTime(rustTimestamp))
    }

    static func monthDay(_ rustTimestamp: String) -> String {
        monthDayFormatter.string(from: parseLogTime(rustTimestamp))
    }
}
