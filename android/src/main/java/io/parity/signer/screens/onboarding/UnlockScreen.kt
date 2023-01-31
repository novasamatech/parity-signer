package io.parity.signer.screens.onboarding

import android.content.res.Configuration
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.fragment.app.FragmentActivity
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.R
import io.parity.signer.components.BigButton
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.findActivity
import io.parity.signer.ui.MainGraphRoutes


/**
 * Initial screen so we won't ask token/password rightaway
 */
fun NavGraphBuilder.unlockAppScreenFlow(globalNavController: NavHostController) {
	composable(route = MainGraphRoutes.initialUnlockRoute) {
		UnlockAppAuthScreen {
			globalNavController.navigate(MainGraphRoutes.mainScreenRoute)
		}
	}
}


@Composable
fun UnlockAppAuthScreen(onSuccess: Callback) {
	val activity = LocalContext.current.findActivity() as FragmentActivity

	Column(verticalArrangement = Arrangement.Center) {
		Spacer(Modifier.weight(0.5f))
		BigButton(
			text = stringResource(R.string.unlock_app_button),
			action = {
				ServiceLocator.authentication.authenticate(activity) {
					onSuccess()
				}
			}
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
