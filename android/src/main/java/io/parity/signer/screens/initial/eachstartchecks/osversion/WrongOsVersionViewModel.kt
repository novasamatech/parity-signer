package io.parity.signer.screens.initial.eachstartchecks.osversion

import android.os.Build
import androidx.lifecycle.ViewModel
import io.parity.signer.domain.DateUtils
import java.util.Calendar


class WrongOsVersionViewModel() : ViewModel() {

	fun isShouldShow(): Boolean {
		return getVulnerabilities().isNotEmpty()
	}
}

//todo dmitry use it
private fun getVulnerabilities(): List<KnownOSIssue> {
 return listOfNotNull(
	 if (is20465Exposed()) KnownOSIssue.CVE_2022_20465 else null,
 )
}

private fun is20465Exposed(): Boolean {
	return if (Build.VERSION.SDK_INT < Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
		val fixPatch = Calendar.getInstance().also {
			it.set(Calendar.YEAR, 2022)
			it.set(Calendar.MONTH, Calendar.NOVEMBER)
		}
		if (DateUtils.parseAndroidPatchDate(Build.VERSION.SECURITY_PATCH)
				?.before(fixPatch) == true) {
			true
		} else {
			false
		}
	} else {
		false
	}
}
enum class KnownOSIssue {
	CVE_2022_20465 // CVE-2022-20465 https://source.android.com/docs/security/bulletin/2022-11-01
}

