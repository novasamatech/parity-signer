package io.parity.signer.domain

import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.flow.*


private class TransformedStateFlow<T>(
	private val getValue: () -> T,
	private val flow: Flow<T>
) : StateFlow<T> {

	override val replayCache: List<T> get() = listOf(value)
	override val value: T get() = getValue()

	override suspend fun collect(collector: FlowCollector<T>): Nothing =
		coroutineScope { flow.stateIn(this).collect(collector) }
}

/**
 * Returns [StateFlow] from [flow] having initial value from calculation of [getValue]
 */
fun <T> stateFlow(
	getValue: () -> T,
	flow: Flow<T>
): StateFlow<T> = TransformedStateFlow(getValue, flow)

/**
 * Combines all [stateFlows] and transforms them into another [StateFlow] with [transform]
 */
inline fun <reified T, R> combineStates(
	vararg stateFlows: StateFlow<T>,
	crossinline transform: (Array<T>) -> R
): StateFlow<R> = stateFlow(
	getValue = { transform(stateFlows.map { it.value }.toTypedArray()) },
	flow = combine(*stateFlows) { transform(it) }
)

/**
 * Variant of [combineStates] for combining 3 state flows
 */
inline fun <reified T1, reified T2, reified T3, R> combineStates(
	flow1: StateFlow<T1>,
	flow2: StateFlow<T2>,
	flow3: StateFlow<T3>,
	crossinline transform: (T1, T2, T3) -> R
) = combineStates(flow1, flow2, flow3) { (t1, t2, t3) ->
	transform(
		t1 as T1,
		t2 as T2,
		t3 as T3
	)
}
