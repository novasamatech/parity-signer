package io.parity.signer.domain

import java.text.ParseException
import java.text.SimpleDateFormat
import java.util.*


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
}

fun Calendar.toLogDateString(): String {
	return SimpleDateFormat("MMM dd", Locale.ENGLISH).format(this.time)
}

fun Calendar.toLogTimeString(): String {
	return SimpleDateFormat("HH:mm", Locale.ENGLISH).format(this.time)
}
