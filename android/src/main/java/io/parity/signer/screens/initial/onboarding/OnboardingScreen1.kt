package io.parity.signer.screens.initial.onboarding

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface


@Composable
internal fun OnboardingScreen1() {
	Column() {
		Row() {

		}
		Text(
			text = "Turn Your Smartphone Into a Hardware Wallet",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
			textAlign = TextAlign.Center,
		)
		Spacer(modifier = Modifier.padding(top = 30.dp))
		Image(
			painter = painterResource(id = R.drawable.onboarding_2),
			contentDescription = null,
			modifier = Modifier
		)
		Spacer(modifier = Modifier.padding(top = 16.dp))
	}
}

@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewOnboarding1() {
	SignerNewTheme {
		OnboardingScreen1()
	}
}
