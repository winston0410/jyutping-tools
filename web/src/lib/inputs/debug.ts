//  const storageOk = (type) => {
//  try {
//  var storage = window[type],
//  x = '__storage_test__';
//  storage.setItem(x, x);
//  storage.removeItem(x);
//  return true;
//  } catch (e) {
//  return false;
//  }
//  };

//  const setInput = (node, keyName) => {
//  const saved = localStorage.getItem(keyName);
//  node.value = saved;
//  };

//  const setRadio = (node, keyName) => {
//  const savedKey = localStorage.getItem(keyName);
//  const targetButton = [...node.elements].find(x => x.value === savedKey)

//  if(targetButton){
//  targetButton.checked = true
//  }
//  }

//  const useLocalStorage = (node, keyName) => {
//  if (!storageOk('localStorage')) {
//  console.warn('localStorage is not supported by the current browser.');
//  return;
//  }

//  console.log(node.tagName.toLowerCase())

//  // TODO Handle CheckBox input
//  if (node instanceof HTMLFieldSetElement) {
//  setRadio(node, keyName);
//  }
//  if (node instanceof HTMLTextAreaElement) {
//  setInput(node, keyName);
//  }

//  const handleChange = (e) => {
//  localStorage.setItem(keyName, e.target.value);
//  };
//  node.addEventListener('change', handleChange);

//  return {
//  destroy() {
//  node.removeEventListener('change', handleChange);
//  }
//  };
//  };
//  export default useLocalStorage;
