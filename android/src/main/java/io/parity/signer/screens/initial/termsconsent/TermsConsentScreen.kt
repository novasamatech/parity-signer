package io.parity.signer.screens.initial.termsconsent

import android.content.res.Configuration.UI_MODE_NIGHT_NO
import android.content.res.Configuration.UI_MODE_NIGHT_YES
import android.util.Log
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.captionBarPadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.components.documents.PpScreen
import io.parity.signer.components.documents.TosScreen
import io.parity.signer.domain.Callback
import io.parity.signer.ui.MainGraphRoutes
import io.parity.signer.ui.NAVIGATION_TAG



@Composable
fun TermsConsentScreenFull(navigateNextScreen: Callback) {
	if (!OnBoardingViewModel.shouldShowOnboarding(LocalContext.current)) {
		navigateNextScreen()
	}

	TermsConsentScreen(
		onBoard = navigateNextScreen,
		modifier = Modifier
			.navigationBarsPadding()
			.captionBarPadding()
			.statusBarsPadding()
	)
}

/**
 * First screen with legal consent request
 */
@Composable
private fun TermsConsentScreen(onBoard: () -> Unit, modifier: Modifier) {
	var state by remember { mutableStateOf(TermsConsentScreenState.GENERAL_SCREEN) }

	Box(modifier = modifier) {
		when (state) {
			TermsConsentScreenState.GENERAL_SCREEN ->
				OnboardingApproveDocumentsScreen(
					onAgree = onBoard,
					onTos = { state = TermsConsentScreenState.TERMS_OF_SERVICE },
					onPp = { state = TermsConsentScreenState.PRIVACY_POLICY },
				)
			TermsConsentScreenState.TERMS_OF_SERVICE ->
				TosScreen(onBack = { state = TermsConsentScreenState.GENERAL_SCREEN })
			TermsConsentScreenState.PRIVACY_POLICY ->
				PpScreen(onBack = { state = TermsConsentScreenState.GENERAL_SCREEN })
		}
	}

	DisposableEffect(key1 = Unit) {
		onDispose { state = TermsConsentScreenState.GENERAL_SCREEN }
	}
}

private enum class TermsConsentScreenState {
	GENERAL_SCREEN, TERMS_OF_SERVICE, PRIVACY_POLICY,
}


@Preview(
	name = "light", group = "themes", uiMode = UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTermsConsentScreen() {
	TermsConsentScreen({}, Modifier)
}
