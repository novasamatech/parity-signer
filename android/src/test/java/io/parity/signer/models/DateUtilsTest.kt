package io.parity.signer.models

import org.junit.Assert.*

import org.junit.Test
import java.text.ParseException
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter
import java.util.*

class DateUtilsTest {

	@Test
	fun parseLogTimeProper() {
		val date = DateUtils.parseLogTime("2014-11-10 13:45")
		val expected = Calendar.Builder().setLocale(Locale.US)
			.setDate(2014, 10, 10) //month is 0 based
			.setTimeOfDay(13, 45, 0)
			.build()
		assertEquals(date!!.get(Calendar.HOUR_OF_DAY), 13)
		assertEquals(date!!.get(Calendar.YEAR), 2014)
		assertEquals(expected, date)
	}

	@Test
	fun parseLogTimeWrong() {
		assertThrows("expected", java.lang.RuntimeException::class.java) {
			val date = DateUtils.parseLogTime("2014/11/10 11:45")
		}
	}

	@Test
	fun parseLogTimeWrong2() {
		assertThrows("expected", java.lang.RuntimeException::class.java) {
			val date = DateUtils.parseLogTime("2914-11-10")
		}
	}

	@Test
	fun dateMatch() {
		val date = DateUtils.parseLogTime("2014-11-10 13:45")!!
		assertEquals("Nov 10", date.toLogDateString())
	}

	@Test
	fun timeMatch() {
		val date = DateUtils.parseLogTime("2014-11-10 13:45")!!
		assertEquals("13:45", date.toLogTimeString())
	}
}
