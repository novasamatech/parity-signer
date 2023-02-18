package io.parity.signer.screens.onboarding.termsconsent

import SignerCheckbox
import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Tab
import androidx.compose.material.TabRow
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CheckboxWithText
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.documents.PpText
import io.parity.signer.components.documents.TacScreen
import io.parity.signer.components.documents.TacText
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.Bg100
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.Typography

@Composable
fun Documents() {
	//todo dmitry implement for onboarding
	var document by remember { mutableStateOf(0) }
	Column {
		//Note to designer:
		//to make the selector pretty, implement
		//custom Tab:
		//https://developer.android.com/reference/kotlin/androidx/compose/material/package-summary#TabRow(kotlin.Int,androidx.compose.ui.Modifier,androidx.compose.ui.graphics.Color,androidx.compose.ui.graphics.Color,kotlin.Function1,kotlin.Function0,kotlin.Function0)
		TabRow(
			selectedTabIndex = document,
			backgroundColor = MaterialTheme.colors.Bg100,
			modifier = Modifier.padding(horizontal = 20.dp, vertical = 10.dp)
		) {
			Tab(
				content = {
					Text(
						text = stringResource(R.string.documents_terms_of_service),
						style = Typography.button
					)
				},
				selected = document == 0,
				onClick = { document = 0 })
			Tab(
				content = {
					Text(
						text = stringResource(R.string.documents_privacy_policy),
						style = Typography.button
					)
				},
				selected = document == 1,
				onClick = { document = 1 })
		}
		Column(
			Modifier
				.verticalScroll(rememberScrollState())
				.padding(20.dp)
		) {
			when (document) {
				0 -> {
//					InstructionsSquare()
					TacText()
				}
				1 -> {
					PpText()
				}
				else -> {
					Text("document selection error")
				}
			}
			Spacer(Modifier.height(150.dp))
		}
	}
}


@Composable
fun OnboardingApproveDocumentsScreen(
	onAgree: Callback,
	onToc: Callback,
	onPp:Callback,
) {
	val isAccepted = remember { mutableStateOf(false) }

	Column(Modifier.fillMaxSize(1f)) {
		Text(
			text = "Please Agree to our Terms of Service and Privacy Policy",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 16.dp)
		)


//		todo dmitry element for keyset area
		Spacer(modifier = Modifier.weight(1f))
		CheckboxWithText(checked = isAccepted.value,
		text = stringResource(R.string.documents_accepted_checkmark),
				) {

		}
		PrimaryButtonWide(label = stringResource(R.string.documents_accept_cta_button)) {

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
private fun PreviewApproveDocumentsScreen() {
	SignerNewTheme {
		//doesn't work in dark mode? Check runtime, it's preview broken for this library
		OnboardingApproveDocumentsScreen({},{},{})
	}
}
