package io.parity.signer.screens.logs

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.models.Navigator
import io.parity.signer.screens.logs.items.LogItem
import io.parity.signer.screens.logs.items.LogItemDate
import io.parity.signer.screens.logs.items.LogsListEntryModel
import io.parity.signer.uniffi.Action

@Composable
fun LogsScreen(
	model: LogsScreenModel,
	navigator: Navigator,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(stringId = R.string.logs_title, onMenu = {
			navigator.navigate(Action.RIGHT_BUTTON_ACTION)
		})
		LazyColumn(Modifier.weight(1f)) {
			items(model.logs.size) { index ->
				when (val item = model.logs[index]) {
					is LogsListEntryModel.LogEntryModel -> {
						LogItem(item) {
							navigator.navigate(
								Action.SHOW_LOG_DETAILS,
								item.logGroupId.toString()
							)
						}
					}
					is LogsListEntryModel.TimeSeparatorModel -> {
						LogItemDate(item)
					}
				}
			}
		}
		BottomBar2(navigator = navigator, state = BottomBar2State.LOGS)
	}
}




