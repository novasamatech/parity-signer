package io.parity.signer.screens.onboarding

import android.content.res.Configuration
import android.util.Log
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.R
import io.parity.signer.components.BigButton
import io.parity.signer.domain.Callback
import io.parity.signer.ui.MainGraphRoutes
import io.parity.signer.ui.NAVIGATION_TAG


/**
 * Initial screen so we won't ask token/password rightaway
 */
fun NavGraphBuilder.initialUnlockAppScreenFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.initialUnlockRoute) {
		UnlockAppAuthScreen {
			globalNavController.navigate(MainGraphRoutes.mainScreenRoute) {
				popUpTo(0)
			}
		}
		LaunchedEffect(Unit) {
			Log.d(NAVIGATION_TAG, "initial unlock screen opened")
		}
	}
}


@Composable
fun UnlockAppAuthScreen(onUnlockClicked: Callback) {
	Column(verticalArrangement = Arrangement.Center) {
		Spacer(Modifier.weight(0.5f))
		BigButton(
			text = stringResource(R.string.unlock_app_button),
			action = { onUnlockClicked() }
		)
		Spacer(Modifier.weight(0.5f))
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
private fun PreviewUnlockAppAuthScreen() {
	UnlockAppAuthScreen {}
}
