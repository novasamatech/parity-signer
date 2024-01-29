package io.parity.signer.screens.error

import android.content.Context
import android.widget.Toast
import androidx.navigation.NavController
import io.parity.signer.R
import io.parity.signer.domain.NavigationError
import io.parity.signer.domain.backend.AuthOperationResult
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.getDebugDetailedDescriptionString
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.uniffi.ErrorDisplayed


data class ErrorStateDestinationState(
	val argHeader: String,
	val argDescription: String,
	val argVerbose: String,
)

inline fun <reified T, E> OperationResult<T, E>.handleErrorAppState(
	coreNavController: NavController
): T? {
	return when (this) {
		is OperationResult.Err -> {
			coreNavController.navigate(
				when (error) {
					is ErrorStateDestinationState -> {
						CoreUnlockedNavSubgraph.ErrorScreenGeneral.destination(
							argHeader = error.argHeader,
							argDescription = error.argDescription,
							argVerbose = error.argVerbose,
						)
					}

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


fun AuthOperationResult.handleErrorAppState(
	coreNavController: NavController,
	context: Context,
): Unit? {
	return when (this) {
		is AuthOperationResult.AuthFailed -> {
			Toast.makeText(context, R.string.auth_failed_message, Toast.LENGTH_SHORT)
				.show()
			null
		}

		is AuthOperationResult.Error -> {
			coreNavController.navigate(
				CoreUnlockedNavSubgraph.ErrorScreenGeneral.destination(
					argHeader = "Operation error",
					argDescription = exception.toString(),
					argVerbose = exception.stackTraceToString(),
				)
			)
			null
		}

		AuthOperationResult.Success -> {
			Unit
		}
	}
}
