package io.parity.signer.screens.settings.logs.logslist

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.navigation.NavController
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.domain.Callback
import io.parity.signer.screens.settings.logs.LogsScreenModel
import io.parity.signer.screens.settings.logs.items.LogItem
import io.parity.signer.screens.settings.logs.items.LogItemDate
import io.parity.signer.screens.settings.logs.items.LogsListEntryModel

@Composable
fun LogsScreen(
	model: LogsScreenModel,
	coreNavController: NavController,
	onMenu: Callback,
	onBack: Callback,
	onLogClicked: (UInt) -> Unit,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(
			title = stringResource(R.string.logs_title),
			onMenu = onMenu,
			onBack = onBack,
		)
		LazyColumn(Modifier.weight(1f)) {
			items(model.logs.size) { index ->
				when (val item = model.logs[index]) {
					is LogsListEntryModel.LogEntryModel -> {
						LogItem(item) {
							onLogClicked(item.logGroupId)
						}
					}
					is LogsListEntryModel.TimeSeparatorModel -> {
						LogItemDate(item)
					}
				}
			}
		}
	}
}




