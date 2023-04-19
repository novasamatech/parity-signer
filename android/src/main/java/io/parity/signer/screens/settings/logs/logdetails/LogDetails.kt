package io.parity.signer.screens.settings.logs.logdetails

import android.util.Log
import android.widget.Toast
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import io.parity.signer.R
import io.parity.signer.backend.CompletableResult
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.domain.Callback
import io.parity.signer.screens.onboarding.WaitingScreen
import io.parity.signer.uniffi.MLogDetails

@Composable
fun LogDetailsScreen(navController: NavController, logDetailsId: UInt) {
	val viewModel: LogsDetailsViewModel = viewModel()
	val context = LocalContext.current

	val logsState = viewModel.logsState.collectAsState()
	val logsCurrentValue = logsState.value

	Box(Modifier.statusBarsPadding()) {
		when (logsCurrentValue) {
			is CompletableResult.Err -> {
				Log.e(TAG, "error in getting log details ${logsCurrentValue.error}")
				Toast.makeText(context, logsCurrentValue.error, Toast.LENGTH_LONG)
					.show()
				viewModel.resetValues()
				navController.popBackStack()
			}
			CompletableResult.InProgress -> {
				WaitingScreen()
			}
			is CompletableResult.Ok -> {
				LogDetails(
					logsCurrentValue.result,
					onBack = { navController.popBackStack() }
				)
			}
		}
		LaunchedEffect(Unit) {
			viewModel.updateLogDetails(logDetailsId)
		}
		DisposableEffect(Unit) {
			onDispose { viewModel.resetValues() }
		}
	}
}

@Composable
private fun LogDetails(logDetails: MLogDetails, onBack: Callback) {
	Column {
		ScreenHeader(
			title = stringResource(R.string.logs_details_title),
			onBack = onBack,
		)
		Text(logDetails.timestamp)
		LazyColumn {
			items(logDetails.events.size) { index ->
				HistoryCardExtended(logDetails.events[index], logDetails.timestamp)
			}
		}
	}
}

private const val TAG = "LogDetailsScreen"
