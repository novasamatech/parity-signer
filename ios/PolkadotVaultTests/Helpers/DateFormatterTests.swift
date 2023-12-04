//
//  DateFormatterTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 01/12/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class DateFormatterTests: XCTestCase {
    func testLogDateFormatterWithValidDateString() {
        // Given
        let dateString = "2023-11-23 15:30:45.123456"
        DateFormatter.logDateFormatter.timeZone = TimeZone(secondsFromGMT: 0)

        // When
        let date = DateFormatter.parseLogTime(dateString)

        // Then
        XCTAssertNotNil(date)
    }

    func testHourMinutesFormatterWithValidDateString() {
        // Given
        let dateString = "2023-11-23 15:30:45.123456"
        let utcTimeZone = TimeZone(secondsFromGMT: 0)
        DateFormatter.logDateFormatter.timeZone = utcTimeZone
        DateFormatter.hourMinutesFormatter.timeZone = utcTimeZone

        // When
        let formattedString = DateFormatter.hourMinutes(dateString)

        // Then
        XCTAssertEqual(formattedString, "15:30")
    }

    func testMonthDayFormatterWithValidDateString() {
        // Given
        let dateString = "2023-11-23 15:30:45.123456"
        DateFormatter.monthDayFormatter.timeZone = TimeZone(secondsFromGMT: 0)

        // When
        let formattedString = DateFormatter.monthDay(dateString)

        // Then
        XCTAssertEqual(formattedString, "Nov 23")
    }

    func testLogDateFormatterWithInvalidDate() {
        // Given
        let invalidDateString = "invalid-date-string"
        DateFormatter.logDateFormatter.timeZone = TimeZone(secondsFromGMT: 0)

        // When
        let date = DateFormatter.parseLogTime(invalidDateString)

        // Then
        XCTAssertNotNil(date) // As the method returns .now for invalid input
    }

    func testHourMinutesFormatterWithInvalidDateString() {
        // Given
        let invalidDateString = "invalid-date-string"
        DateFormatter.hourMinutesFormatter.timeZone = TimeZone(secondsFromGMT: 0)

        // When
        let formattedString = DateFormatter.hourMinutes(invalidDateString)

        // Then
        XCTAssertEqual(formattedString, DateFormatter.hourMinutesFormatter.string(from: Date()))
    }

    func testMonthDayFormatterWithInvalidDateString() {
        // Given
        let invalidDateString = "invalid-date-string"
        DateFormatter.monthDayFormatter.timeZone = TimeZone(secondsFromGMT: 0)

        // When
        let formattedString = DateFormatter.monthDay(invalidDateString)

        // Then
        XCTAssertEqual(formattedString, DateFormatter.monthDayFormatter.string(from: Date()))
    }
}
