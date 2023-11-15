package io.parity.signer.screens.error

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.domain.NavigationError
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.backend.toOperationResult
import io.parity.signer.domain.getDebugDetailedDescriptionString
import io.parity.signer.screens.error.wrongversion.errorWrongVersionSubgraph
import io.parity.signer.screens.initial.eachstartchecks.airgap.AirgapScreen
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.uniffi.ErrorDisplayed


fun NavGraphBuilder.errorStateDestination(
	navController: NavController,
) {
	composable(
		route = CoreUnlockedNavSubgraph.ErrorScreenGeneral.route,
		arguments = listOf(
			navArgument(CoreUnlockedNavSubgraph.ErrorScreenGeneral.argHeader) {
				type = NavType.StringType
			},
			navArgument(CoreUnlockedNavSubgraph.ErrorScreenGeneral.argDescription) {
				type = NavType.StringType
			},
			navArgument(CoreUnlockedNavSubgraph.ErrorScreenGeneral.argVerbose) {
				type = NavType.StringType
			},
		),
	) {
		val argHeader =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorScreenGeneral.argHeader)!!
		val argDescr =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorScreenGeneral.argDescription)!!
		val argVerbose =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorScreenGeneral.argVerbose)!!

		ErrorStateScreen(
			header = argHeader,
			description = argDescr,
			verbose = argVerbose,
			onBack = { navController.popBackStack() },
			modifier = Modifier.statusBarsPadding()
		)
	}
	errorWrongVersionSubgraph(navController)
	composable(route = CoreUnlockedNavSubgraph.airgapBreached) {
		BackHandler {
			//disable back navigation on this screen
		}
		AirgapScreen(isInitialOnboarding = false) {
			navController.popBackStack()
		}
	}
}


inline fun <reified T> UniffiResult<T>.handleErrorAppState(coreNavController: NavController): T? {
	return this.toOperationResult().handleErrorAppState(coreNavController)
}


inline fun <reified T, E> OperationResult<T, E>.handleErrorAppState(
	coreNavController: NavController
): T? {
	return when (this) {
		is OperationResult.Err -> {
			coreNavController.navigate(
				when (error) {
					is NavigationError -> {
						CoreUnlockedNavSubgraph.ErrorScreenGeneral.destination(
							argHeader = "Operation navigation error trying to get ${T::class.java}",
							argDescription = error.message,
							argVerbose = "",
						)
					}

					is ErrorDisplayed ->
						when (error) {
							is ErrorDisplayed.DbSchemaMismatch -> {
								CoreUnlockedNavSubgraph.errorWrongDbVersionUpdate
							}
							else -> {
								CoreUnlockedNavSubgraph.ErrorScreenGeneral.destination(
									argHeader = "Operation error to get ${T::class.java}",
									argDescription = error.toString(),
									argVerbose = error.getDebugDetailedDescriptionString(),
								)
							}
						}

					else -> {
						CoreUnlockedNavSubgraph.ErrorScreenGeneral.destination(
							argHeader = "Operation unknown error trying to get ${T::class.java}",
							argDescription = "",
							argVerbose = error.toString(),
						)
					}
				}
			)
			null
		}

		is OperationResult.Ok -> {
			result
		}
	}
}
