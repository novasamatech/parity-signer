process.browser = true

if (typeof global.Buffer !== 'undefined') {
  // running on VSCode debugger
  global.Buffer = undefined
}

require('crypto')