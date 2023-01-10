package io.parity.signer.screens.keyderivation

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.HelpOutline
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.components.base.DotsIndicator
import io.parity.signer.components.base.PrimaryButtonGreyDisabled
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.*


@Composable
fun DerivationPathScreen(
	onDerivationHelp: Callback,
	onClose: Callback,
	onDone: Callback,
) {
	val canProceed = true
	val path = remember {
		mutableStateOf("//")
	}

	val focusManager = LocalFocusManager.current
	val focusRequester = remember { FocusRequester() }

	val onDoneLocal = {
		onDone()
		focusManager.clearFocus(true)
	}

	Column() {
		DerivationPathHeader(
			canProceed = canProceed,
			onClose = onClose,
			onDone = onDoneLocal,
		)
		OutlinedTextField(
			value = path.value, //hide password, add hint
			onValueChange = { newStr -> path.value = newStr },
			keyboardOptions = KeyboardOptions(
				imeAction = if (canProceed) ImeAction.Done else ImeAction.None
			),
			visualTransformation = DerivationPathVisualTransformation(
				LocalContext.current,
				MaterialTheme.colors
			),
			keyboardActions = KeyboardActions(
				onDone = {
					if (canProceed) {
						onDoneLocal()
					}
				}
			),
			singleLine = true,
			textStyle = SignerTypeface.LabelM,
			colors = TextFieldDefaults.textFieldColors(textColor = MaterialTheme.colors.primary),
			modifier = Modifier
				.focusRequester(focusRequester)
				.fillMaxWidth(1f)
				.padding(horizontal = 24.dp)
		)
		//suggestion slashes
//		val hintBackground = MaterialTheme.colors.fill6 + MaterialTheme.colors.backgroundDanger
		Row(
			Modifier
				.padding(top = 8.dp, bottom = 16.dp)
				.padding(horizontal = 24.dp)
		) {
			Button(
				shape = RoundedCornerShape(24.dp),
				colors = ButtonDefaults.buttonColors(backgroundColor = MaterialTheme.colors.backgroundDanger),
				onClick = { path.value = path.value + "/" },
			) {
				Text(text = "/")
			}
			Button(
				shape = RoundedCornerShape(24.dp),
				colors = ButtonDefaults.buttonColors(backgroundColor = MaterialTheme.colors.backgroundDanger),
				onClick = { path.value = path.value + "//" },
			) {
				Text(text = "//")
			}
			Button(
				shape = RoundedCornerShape(24.dp),
				colors = ButtonDefaults.buttonColors(backgroundColor = MaterialTheme.colors.backgroundDanger),
				onClick = { path.value = path.value + "///" },
			) {
				Text(text = "/// Password")
			}
		}
		Text(
			text = "Path name example //network//1",
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier.padding(horizontal = 24.dp)
		)
		DerivationAlarm(
			Modifier
				.padding(top = 16.dp, bottom = 16.dp)
				.padding(horizontal = 24.dp)
				.clickable(onClick = onDerivationHelp)
		)
	}
}

@Composable
private fun DerivationAlarm(modifier: Modifier = Modifier) {
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Row(
		modifier = modifier
			.padding(vertical = 8.dp)
			.border(
				BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
				innerShape
			)

	) {
		Text(
			text = "Learn more about /soft, //hard, ///password derivation methods",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyM,
			modifier = Modifier
				.weight(1f)
				.padding(start = 16.dp, top = 16.dp, bottom = 16.dp)
		)
		Icon(
			imageVector = Icons.Outlined.HelpOutline,
			contentDescription = null,
			tint = MaterialTheme.colors.pink300,
			modifier = Modifier
				.align(Alignment.CenterVertically)
				.padding(start = 18.dp, end = 18.dp)
		)
	}
}

/**
 * io/parity/signer/screens/keysets/create/NewKeySetNameScreen.kt:107
 */
@Composable
private fun DerivationPathHeader(
	canProceed: Boolean,
	onClose: Callback,
	onDone: Callback,
) {
	Box(
		modifier = Modifier
			.padding(start = 24.dp, end = 8.dp, top = 8.dp, bottom = 8.dp),
		contentAlignment = Alignment.Center,
	) {
		Box(
			modifier = Modifier.fillMaxWidth(1f),
			contentAlignment = Alignment.CenterStart,
		) {
			CloseIcon(
				modifier = Modifier.padding(vertical = 20.dp),
				noBackground = true,
				onCloseClicked = onClose,
			)
		}
		Box(
			modifier = Modifier.fillMaxWidth(1f),
			contentAlignment = Alignment.Center,
		) {
			DotsIndicator(totalDots = 2, selectedIndex = 1)
		}
		Box(
			modifier = Modifier.fillMaxWidth(1f),
			contentAlignment = Alignment.CenterEnd,
		) {
			PrimaryButtonGreyDisabled(
				label = stringResource(R.string.generic_done),
				isEnabled = canProceed,
			) {
				if (canProceed) {
					onDone()
				}
			}
		}
	}
}

@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewDerivationPathScreen() {
	SignerNewTheme {
		DerivationPathScreen({}, {}, {})
	}
}
