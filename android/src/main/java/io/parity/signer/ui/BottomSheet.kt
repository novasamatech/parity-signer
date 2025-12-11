package io.parity.signer.ui

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clipToBounds
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.backgroundTertiary
import kotlinx.coroutines.launch

/**
 * For use in the same screen with content
 * .navigationBarsPadding().captionBarPadding() paddings should be added already
 */
@OptIn(ExperimentalMaterialApi::class)
@Composable
fun BottomSheetWrapperContent(
	bottomSheetState: ModalBottomSheetState,
	bottomSheetContent: @Composable () -> Unit,
	mainContent: @Composable () -> Unit
) {
	val scope = rememberCoroutineScope()

	BackHandler(enabled = bottomSheetState.isVisible) {
		scope.launch { bottomSheetState.hide() }
	}

	ModalBottomSheetLayout(
		modifier = Modifier.clipToBounds(),
		sheetBackgroundColor = MaterialTheme.colors.backgroundTertiary,
		sheetShape = RoundedCornerShape(topEnd = 16.dp, topStart = 16.dp),
		sheetState = bottomSheetState,
		sheetElevation = 0.dp,
		sheetContent = {
			BottomSheetContentWrapperInternal {
				bottomSheetContent()
			}
		},
		content = {
			Box(Modifier.statusBarsPadding()) {
				mainContent()
			}
		}
	)
}

/**
 * Used for screens controlled by central rust-based navigation system
 */
@OptIn(ExperimentalMaterialApi::class)
@Composable
fun BottomSheetWrapperRoot(
	onClosedAction: () -> Unit = {},
	bottomSheetContent: @Composable (state: BottomSheetPositionHandle) -> Unit,
) {
	val coroutineScope = rememberCoroutineScope()
	val modalBottomSheetState =
		rememberModalBottomSheetState(
			ModalBottomSheetValue.Hidden,
			skipHalfExpanded = true,
			confirmValueChange = {
				it != ModalBottomSheetValue.HalfExpanded
			}
		)

	var wasSheetClosed by remember { mutableStateOf(false) }
	var wasSheetShown by remember { mutableStateOf(false) }

	val handle = remember {
		object : BottomSheetPositionHandle {
			override fun hide() {
				wasSheetClosed = true
				coroutineScope.launch {
					if (modalBottomSheetState.isVisible) {
						modalBottomSheetState.hide()
					}
				}
			}

			override fun show() {
				coroutineScope.launch {
					modalBottomSheetState.show()
				}
			}
		}
	}

	ModalBottomSheetLayout(
		sheetBackgroundColor = MaterialTheme.colors.backgroundTertiary,
		sheetShape = RoundedCornerShape(topEnd = 16.dp, topStart = 16.dp),
		sheetState = modalBottomSheetState,
		sheetElevation = 0.dp,
		sheetContent = {
			BottomSheetContentWrapperInternal {
				bottomSheetContent(handle)
			}
		},
		content = {},
	)

	BackHandler(enabled = modalBottomSheetState.isVisible) {
		coroutineScope.launch { modalBottomSheetState.hide() }
	}

	//show once view is create to have initial open animation
	LaunchedEffect(key1 = modalBottomSheetState) {
		modalBottomSheetState.show()
		//sometimes never happen. Show suspends forever like for password dialog
	}

	// Take action based on hidden state
	LaunchedEffect(modalBottomSheetState.currentValue) {
		when (modalBottomSheetState.currentValue) {
			ModalBottomSheetValue.Hidden -> {
				if (!wasSheetClosed && wasSheetShown) {
					onClosedAction()
				}
			}
			else -> {
				wasSheetShown = true
			}
		}
	}
}

interface BottomSheetPositionHandle {
	fun hide()
	fun show()
}

@Composable
private fun BottomSheetContentWrapperInternal(
	content: @Composable () -> Unit,
) {
	val screenHeight = LocalConfiguration.current.screenHeightDp.dp
	Box(
		modifier = Modifier
			.wrapContentHeight()
			.heightIn(0.dp, screenHeight - 40.dp)
			.fillMaxWidth()
	) {
		content()
	}
}



