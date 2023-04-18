package io.parity.signer.screens.settings.logs.comment

import androidx.compose.foundation.layout.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.domain.Callback


@Composable
internal fun AddLogCommentScreen(onBack:Callback, ) {
	Column(Modifier.statusBarsPadding()) {
		ScreenHeaderWithButton(title = stringResource(R.string.add_log_comment_title))
		Spacer(modifier = Modifier.padding(top = 24.dp))
		//todo dmitry finish
	}
}
