/**
 * Creates and returns a new debounced version of the passed function that will
 * postpone its execution until after wait milliseconds have elapsed since
 * the last time it was invoked.
 *
 * @type  {T}                item    type
 * @param {(any) => any}     function to debounce
 * @param {number}           time in milliseconds
 *
 *
 * @return {any}            the debounced function
 */
export function debounce(fn, time) {
	let timeout;

	return function() {
		const functionCall = () => fn.apply(this, arguments);

		clearTimeout(timeout);
		timeout = setTimeout(functionCall, time);
	};
}
