package io.parity.signer.domain

import java.text.ParseException
import java.text.SimpleDateFormat
import java.util.Calendar
import java.util.Date
import java.util.Locale


object DateUtils {
	fun parseLogTime(rustDateTimeString: String): Calendar? {
		return try {
			val calendar = Calendar.getInstance()
			val sdf = SimpleDateFormat("yyyy-MM-dd HH:mm", Locale.ENGLISH)
			calendar.time = sdf.parse(rustDateTimeString) ?: return null
			calendar
		} catch (e: ParseException) {
			submitErrorState("cannot parse date from rust, $e")
			null
		}
	}

	fun parseAndroidPatchDate(version: String): Calendar? {
		//last 2 numbers is not day but actually 2 types of patches. But we will check only months
		//and year is YYYY not yyyy yet since we don't check day, it should be fine. YYYY have minsdk 24
		val sdf = SimpleDateFormat("yyyy-MM-DD", Locale.ENGLISH)
		val calendar = Calendar.getInstance()
		return try {
			calendar.time = sdf.parse(version) ?: throw ParseException("NUll date in return", -1)
			calendar
		} catch (e: ParseException) {
			submitErrorState("Wrong android patch version format?? $e")
			null
		}
	}
}

fun Calendar.toLogDateString(): String {
	return SimpleDateFormat("MMM dd", Locale.ENGLISH).format(this.time)
}

fun Calendar.toLogTimeString(): String {
	return SimpleDateFormat("HH:mm", Locale.ENGLISH).format(this.time)
}
