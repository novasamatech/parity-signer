package io.parity.signer.screens.scan.items

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary

@Composable
fun ScanErrorBottomSheet(
	errorMessage: String,
	onClose: Callback,
) {
	var password by rememberSaveable { mutableStateOf("") }

	Column(
		modifier = Modifier
	) {
		Column(
			modifier = Modifier
                .padding(horizontal = 24.dp)
                .verticalScroll(
                    rememberScrollState()
                )
		) {
			Text(
				text = stringResource(R.string.enter_password_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
			)
			Spacer(modifier = Modifier.padding(top = 16.dp))
//		if (enterPassword.counter > 0u) {
//			Text("Attempt " + enterPassword.counter.toString() + " of 3")
//		}

			Spacer(modifier = Modifier.padding(top = 16.dp))

			Text(
				text = stringResource(R.string.enter_password_description),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.CaptionM,
				modifier = Modifier.padding(top = 8.dp, bottom = 30.dp),
			)
		}
	}
}




@Preview(
	name = "day",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark theme",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewScanErrorBottomSheet() {
	SignerNewTheme {
		ScanErrorBottomSheet("My super error", {})
	}
}
