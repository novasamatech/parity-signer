package io.parity.signer.screens.scan.transaction

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.domain.Callback
import io.parity.signer.screens.scan.transaction.components.TransactionElementSelector
import io.parity.signer.ui.theme.backgroundPrimary
import io.parity.signer.uniffi.MTransaction


@Composable
internal fun TransactionDetailsScreen(
	transaction: MTransaction,
	modifier: Modifier = Modifier,
	onBack: Callback
) {
	BackHandler(onBack = onBack)

	Column(
		modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.backgroundPrimary)
	) {
		ScreenHeader(
			title = stringResource(R.string.transaction_details_screen_header),
			onBack = onBack
		)
		Column(
			Modifier.verticalScroll(rememberScrollState())
		) {
			TransactionIssues(transaction)
			transaction.sortedValueCards.forEach {
				TransactionElementSelector(it)
			}
			Spacer(modifier = modifier.padding(top = 24.dp))
		}
	}
}
