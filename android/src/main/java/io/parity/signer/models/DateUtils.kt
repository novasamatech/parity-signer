package io.parity.signer.models

import java.text.ParseException
import java.text.SimpleDateFormat
import java.util.*

object DateUtils {
	fun parseLogTime(rustDateTimeString: String): Calendar? {
		return try {
			val calendar = Calendar.getInstance()
			val sdf = SimpleDateFormat("yyyy-MM-dd hh:mm", Locale.US)
			calendar.time = sdf.parse(rustDateTimeString) ?: return null
			calendar
		} catch (e: ParseException) {
			submitErrorState("cannot parse date from rust, $e")
			null
		}
	}
}
