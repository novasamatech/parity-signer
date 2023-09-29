package io.parity.signer.screens.error

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.domain.NavigationError
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.getDebugDetailedDescriptionString
import io.parity.signer.domain.submitErrorState
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.uniffi.ErrorDisplayed


fun NavGraphBuilder.errorStateDestination(
	navController: NavController,
) {
	composable(
		route = CoreUnlockedNavSubgraph.ErrorScreen.route,
		arguments = listOf(
			navArgument(CoreUnlockedNavSubgraph.ErrorScreen.argHeader) {
				type = NavType.StringType
			},
			navArgument(CoreUnlockedNavSubgraph.ErrorScreen.argDescription) {
				type = NavType.StringType
			},
			navArgument(CoreUnlockedNavSubgraph.ErrorScreen.argVerbose) {
				type = NavType.StringType
			},
		),
	) {
		val argHeader =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorScreen.argHeader)!!
		val argDescr =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorScreen.argDescription)!!
		val argVerbose =
			it.arguments?.getString(CoreUnlockedNavSubgraph.ErrorScreen.argVerbose)!!

		ErrorStateScreen(
			header = argHeader,
			description = argDescr,
			verbose = argVerbose,
			onBack = { navController.popBackStack() },
		)
	}
}


inline fun <reified T> UniffiResult<T>.handleErrorAppState(coreNavController: NavController): T? {
	return when (this) {
		is UniffiResult.Error -> {
			coreNavController.navigate(
				CoreUnlockedNavSubgraph.ErrorScreen.destination(
					argHeader = "Uniffi interaction error trying to get ${T::class.java}",
					argDescription = error.toString(),
					argVerbose = error.getDebugDetailedDescriptionString(),
				)
			)
			null
		}

		is UniffiResult.Success -> {
			result
		}
	}
}


inline fun <reified T, E> OperationResult<T, E>.handleErrorAppState(
	coreNavController: NavController
): T? {
	return when (this) {
		is OperationResult.Err -> {
			coreNavController.navigate(
				when (error) {
					is NavigationError -> {
						CoreUnlockedNavSubgraph.ErrorScreen.destination(
							argHeader = "Operation navigation error trying to get ${T::class.java}",
							argDescription = error.message,
							argVerbose = "",
						)
					}
					is ErrorDisplayed -> {
						CoreUnlockedNavSubgraph.ErrorScreen.destination(
							argHeader = "Operation error to get ${T::class.java}",
							argDescription = error.toString(),
							argVerbose = error.getDebugDetailedDescriptionString(),
						)
					}
					else -> {
						CoreUnlockedNavSubgraph.ErrorScreen.destination(
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
