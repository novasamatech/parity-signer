package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import io.parity.signer.uniffi.MscEraMortal
import org.json.JSONObject

@Composable
fun TCEra(era: MscEraMortal) {
	Column {
		TCNameValueTemplate(name = "phase", value = era.phase)
		TCNameValueTemplate(name = "period", value = era.period)
	}
}
