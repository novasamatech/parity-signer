package io.parity.signer.screens.scan.items

import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Visibility
import androidx.compose.material.icons.filled.VisibilityOff
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.components.base.PrimaryButtonGreyDisabled
import io.parity.signer.components.sharedcomponents.KeyCardModelBase
import io.parity.signer.components.sharedcomponents.KeyCardPassword
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.red500
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.MEnterPassword

@Composable
fun ScanErrorBottomSheet(
	errorMessage: String,
	onClose: Callback,
) {
	var password by rememberSaveable { mutableStateOf("") }
	var passwordVisible by rememberSaveable { mutableStateOf(false) }

	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	val canProceed = password.isNotBlank()

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
