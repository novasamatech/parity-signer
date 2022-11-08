package io.parity.signer.screens.logs

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.models.Navigator
import io.parity.signer.models.navigate
import io.parity.signer.screens.logs.items.LogItem
import io.parity.signer.screens.logs.items.LogItemDate
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.History


@Composable
fun LogsScreen(
	model: LogsScreenModel,
	navigator: Navigator,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(stringId = R.string.logs_title, onMenu = {
			navigator.navigate(Action.RIGHT_BUTTON_ACTION)
		})
		Box() {
			LazyColumn {
				items(
					items = model.logs,
					key = { it.hashCode() }
				) { item ->
					when (item) {
						is LogsListEntryModel.LogEntryModel -> {
							Box(
								Modifier.clickable {
									navigator.navigate(
										Action.SHOW_LOG_DETAILS,
										item.logGroupId.toString()
									)
								}
							) {
								//todo dmitry
//								LogItem(title = item., message =, time =)
							}
						}
						is LogsListEntryModel.TimeSeparatorModel -> {
							//todo dmitry
//							LogItemDate(date = item.)
						}
					}
				}
			}
		}
		BottomBar2(navigator = navigator, state = BottomBar2State.LOGS)
	}
}




