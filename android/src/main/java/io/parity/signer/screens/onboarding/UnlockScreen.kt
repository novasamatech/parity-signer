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
import androidx.navigation.compose.composable
import io.parity.signer.R
import io.parity.signer.components.BigButton
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.findActivity


const val unlockAppScreenRoute = "navigation_point_unlock_app"//

fun NavGraphBuilder.unlockAppScreenFlow() {
	composable(route = unlockAppScreenRoute) {
		val model: SignerDataModel = viewModel() //todo onboarding remove
		UnlockAppAuthScreen(model)
	}
}



@Composable
private fun UnlockAppAuthScreen(signerDataModel: SignerDataModel) {
	val activity = LocalContext.current.findActivity() as FragmentActivity

	Column(verticalArrangement = Arrangement.Center) {
		Spacer(Modifier.weight(0.5f))
		BigButton(
			text = stringResource(R.string.unlock_app_button),
			action = {
				ServiceLocator.authentication.authenticate(activity) {
					signerDataModel.totalRefresh()
				}
			}
		)
		Spacer(Modifier.weight(0.5f))
	}
}
