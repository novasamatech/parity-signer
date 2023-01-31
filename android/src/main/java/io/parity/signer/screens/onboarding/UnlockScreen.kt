package io.parity.signer.screens.onboarding

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavHostController
import androidx.navigation.compose.composable
import io.parity.signer.R
import io.parity.signer.components.BigButton
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.MainFlowViewModel
import io.parity.signer.domain.findActivity
import io.parity.signer.ui.mainScreenRoute


const val unlockAppScreenRoute = "navigation_point_unlock_app"

fun NavGraphBuilder.unlockAppScreenFlow(globalNavController: NavHostController) {
	composable(route = unlockAppScreenRoute) {
		val model: MainFlowViewModel = viewModel() //todo onboarding remove
		UnlockAppAuthScreen {
			globalNavController.navigate(mainScreenRoute)
			model.totalRefresh() //todo onboarding remove
		}
	}
}


@Composable
private fun UnlockAppAuthScreen(onSuccess: Callback) {
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
