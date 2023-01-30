package io.parity.signer.screens.onboarding

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import io.parity.signer.R
import io.parity.signer.components.BigButton
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.SignerDataModel


const val unlockAppScreenRoute = "navigation_point_unlock_app"//

fun NavGraphBuilder.unlockAppScreenFlow() {
	composable(route = unlockAppScreenRoute) {
		UnlockAppAuthScreen()
	}
}



@Composable
private fun UnlockAppAuthScreen(signerDataModel: SignerDataModel) {
	Column(verticalArrangement = Arrangement.Center) {
		Spacer(Modifier.weight(0.5f))
		BigButton(
			text = stringResource(R.string.unlock_app_button),
			action = {
				ServiceLocator.authentication.authenticate(signerDataModel.activity) {
					signerDataModel.totalRefresh()
				}
			}
		)
		Spacer(Modifier.weight(0.5f))
	}
}
