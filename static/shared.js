// Token 管理功能
/**
 * 保存认证令牌到本地存储
 * @param {string} token - 要保存的认证令牌
 * @returns {void}
 */
function saveAuthToken(token) {
  const expiryTime = new Date().getTime() + 24 * 60 * 60 * 1000; // 24小时后过期
  localStorage.setItem("authToken", token);
  localStorage.setItem("authTokenExpiry", expiryTime);
}

/**
 * 获取存储的认证令牌
 * @returns {string|null} 如果令牌有效则返回令牌，否则返回 null
 */
function getAuthToken() {
  const token = localStorage.getItem("authToken");
  const expiry = localStorage.getItem("authTokenExpiry");

  if (!token || !expiry) {
    return null;
  }

  if (new Date().getTime() > parseInt(expiry)) {
    localStorage.removeItem("authToken");
    localStorage.removeItem("authTokenExpiry");
    return null;
  }

  return token;
}

// 消息显示功能
/**
 * 在指定元素中显示消息
 * @param {string} elementId - 目标元素的 ID
 * @param {string} text - 要显示的消息文本
 * @param {boolean} [isError=false] - 是否为错误消息
 * @returns {void}
 */
function showMessage(elementId, text, isError = false) {
  let msg = document.getElementById(elementId);

  // 如果消息元素不存在，创建一个新的
  if (!msg) {
    msg = document.createElement("div");
    msg.id = elementId;
    document.body.appendChild(msg);
  }

  msg.className = `floating-message ${isError ? "error" : "success"}`;
  msg.innerHTML = text.replace(/\n/g, "<br>");
}

// 确保消息容器存在
/**
 * 确保消息容器存在于 DOM 中
 * @returns {HTMLElement} 消息容器元素
 */
function ensureMessageContainer() {
  let container = document.querySelector(".message-container");
  if (!container) {
    container = document.createElement("div");
    container.className = "message-container";
    document.body.appendChild(container);
  }
  return container;
}

/**
 * 显示全局消息提示
 * @param {string} text - 要显示的消息文本
 * @param {boolean} [isError=false] - 是否为错误消息
 * @param {number} [timeout=3000] - 消息显示时长（毫秒）
 * @returns {void}
 */
function showGlobalMessage(text, isError = false, timeout = 3000) {
  const container = ensureMessageContainer();

  const msgElement = document.createElement("div");
  msgElement.className = `message ${isError ? "error" : "success"}`;
  msgElement.textContent = text;

  container.appendChild(msgElement);

  // 设置淡出动画和移除
  setTimeout(() => {
    msgElement.style.animation = "messageOut 0.3s ease-in-out";
    setTimeout(() => {
      msgElement.remove();
      // 如果容器为空，也移除容器
      if (container.children.length === 0) {
        container.remove();
      }
    }, 300);
  }, timeout);
}

// Token 输入框自动填充和事件绑定
function initializeTokenHandling(inputId) {
  // 直接尝试填充，如果DOM未准备好会在事件中再试一次
  const tryFillToken = () => {
    const tokenInput = document.getElementById(inputId);
    if (tokenInput) {
      const authToken = getAuthToken();
      if (authToken) {
        tokenInput.value = authToken;
      }

      // 绑定change事件
      tokenInput.addEventListener("change", (e) => {
        if (e.target.value) {
          saveAuthToken(e.target.value);
        } else {
          localStorage.removeItem("authToken");
          localStorage.removeItem("authTokenExpiry");
        }
      });

      return true;
    }
    return false;
  };

  // 立即尝试执行
  if (!tryFillToken()) {
    // 如果元素还不存在，等待DOM加载完成
    if (document.readyState === 'loading') {
      document.addEventListener("DOMContentLoaded", tryFillToken);
    } else {
      // DOM已加载但元素不存在，可能需要等待一下
      setTimeout(tryFillToken, 0);
    }
  }
}

// API 请求通用处理
async function makeAuthenticatedRequest(url, options = {}) {
  const tokenId = options.tokenId || "authToken";
  const token = document.getElementById(tokenId).value;

  if (!token) {
    showGlobalMessage("请输入 AUTH_TOKEN", true);
    return null;
  }

  if (!/^[A-Za-z0-9\-._~+/]+=*$/.test(token)) {
    showGlobalMessage("TOKEN格式无效，请检查是否包含特殊字符", true);
    return null;
  }

  const defaultOptions = {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  };

  try {
    const response = await fetch(url, { ...defaultOptions, ...options });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  } catch (error) {
    showGlobalMessage(`请求失败: ${error.message}`, true);
    return null;
  }
}

/**
 * 从字符串解析布尔值
 * @param {string} str - 要解析的字符串
 * @param {boolean|null} defaultValue - 解析失败时的默认值
 * @returns {boolean|null} 解析结果，如果无法解析则返回默认值
 */
function parseBooleanFromString(str, defaultValue = null) {
  if (typeof str !== "string") {
    return defaultValue;
  }

  const lowercaseStr = str.toLowerCase().trim();

  if (lowercaseStr === "true" || lowercaseStr === "1") {
    return true;
  } else if (lowercaseStr === "false" || lowercaseStr === "0") {
    return false;
  } else {
    return defaultValue;
  }
}

/**
 * 将布尔值转换为字符串
 * @param {boolean|undefined|null} value - 要转换的布尔值
 * @param {string} defaultValue - 转换失败时的默认值
 * @returns {string} 转换结果，如果输入无效则返回默认值
 */
function parseStringFromBoolean(value, defaultValue = null) {
  if (typeof value !== "boolean") {
    return defaultValue;
  }

  return value ? "true" : "false";
}

/**
 * 将会员类型代码转换为显示名称
 * @param {string|null} type - 会员类型代码,如 'free_trial', 'pro', 'free', 'enterprise' 等
 * @returns {string} 格式化后的会员类型显示名称
 * @example
 * formatMembershipType('free_trial') // 返回 'Pro Trial'
 * formatMembershipType('pro') // 返回 'Pro'
 * formatMembershipType(null) // 返回 '-'
 * formatMembershipType('custom_type') // 返回 'Custom Type'
 */
function formatMembershipType(type) {
  if (!type) return "-";
  switch (type) {
    case "free_trial":
      return "Pro Trial";
    case "pro":
      return "Pro";
    case "free":
      return "Free";
    case "enterprise":
      return "Business";
    default:
      return type
        .split("_")
        .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
        .join(" ");
  }
}

// 复制文本功能
/**
 * 复制文本到剪贴板
 * @param {string} text - 要复制的文本
 * @param {Object} [options={}] - 配置选项
 * @param {boolean} [options.showMessage=true] - 是否显示复制结果消息
 * @param {string} [options.successMessage='已复制到剪贴板'] - 复制成功时的消息
 * @param {string} [options.errorMessage='复制失败，请手动复制'] - 复制失败时的消息
 * @param {Function} [options.onSuccess] - 复制成功时的回调函数
 * @param {Function} [options.onError] - 复制失败时的回调函数
 * @param {HTMLElement} [options.sourceElement] - 触发复制的源元素（用于显示临时状态）
 * @returns {Promise<boolean>} 返回复制是否成功
 * @example
 * // 基础用法
 * copyToClipboard('Hello World');
 *
 * // 自定义消息
 * copyToClipboard('代理地址', {
 *   successMessage: '代理地址已复制',
 *   errorMessage: '无法复制代理地址'
 * });
 *
 * // 带回调函数
 * copyToClipboard('敏感信息', {
 *   showMessage: false,
 *   onSuccess: () => console.log('复制成功'),
 *   onError: (err) => console.error('复制失败:', err)
 * });
 *
 * // 与按钮配合使用
 * const button = document.getElementById('copyBtn');
 * copyToClipboard('文本内容', { sourceElement: button });
 */
async function copyToClipboard(text, options = {}) {
  const {
    showMessage = true,
    successMessage = "已复制到剪贴板",
    errorMessage = "复制失败，请手动复制",
    onSuccess,
    onError,
    sourceElement,
  } = options;

  // 验证输入
  if (typeof text !== "string") {
    console.error("copyToClipboard: 文本必须是字符串类型");
    if (showMessage) {
      showGlobalMessage("无效的复制内容", true);
    }
    if (onError) {
      onError(new Error("Invalid text type"));
    }
    return false;
  }

  // 如果文本为空，给出警告
  if (!text.trim()) {
    console.warn("copyToClipboard: 尝试复制空文本");
    if (showMessage) {
      showGlobalMessage("没有可复制的内容", true);
    }
    if (onError) {
      onError(new Error("Empty text"));
    }
    return false;
  }

  try {
    // 优先使用现代 Clipboard API
    if (navigator.clipboard && window.isSecureContext) {
      await navigator.clipboard.writeText(text);
      handleCopySuccess();
      return true;
    } else {
      // 降级到传统方法
      const success = fallbackCopyToClipboard(text);
      if (success) {
        handleCopySuccess();
        return true;
      } else {
        throw new Error("Fallback copy failed");
      }
    }
  } catch (error) {
    console.error("复制到剪贴板失败:", error);

    if (showMessage) {
      showGlobalMessage(errorMessage, true);
    }

    if (onError) {
      onError(error);
    }

    return false;
  }

  // 处理复制成功
  function handleCopySuccess() {
    if (showMessage) {
      showGlobalMessage(successMessage);
    }

    if (onSuccess) {
      onSuccess();
    }

    // 如果提供了源元素，可以添加临时的视觉反馈
    if (sourceElement) {
      addTemporaryClass(sourceElement, "copied", 2000);
    }
  }
}

/**
 * 传统的复制方法（用于不支持 Clipboard API 的浏览器）
 * @private
 * @param {string} text - 要复制的文本
 * @returns {boolean} 是否复制成功
 */
function fallbackCopyToClipboard(text) {
  // 创建临时文本区域
  const textArea = document.createElement("textarea");

  // 设置样式使其不可见但可复制
  textArea.value = text;
  textArea.style.position = "fixed";
  textArea.style.top = "0";
  textArea.style.left = "0";
  textArea.style.width = "2em";
  textArea.style.height = "2em";
  textArea.style.padding = "0";
  textArea.style.border = "none";
  textArea.style.outline = "none";
  textArea.style.boxShadow = "none";
  textArea.style.background = "transparent";
  textArea.style.opacity = "0";
  textArea.style.pointerEvents = "none";

  // 防止移动设备上的缩放
  textArea.style.fontSize = "12pt";

  document.body.appendChild(textArea);

  try {
    // 选择文本
    textArea.select();
    textArea.setSelectionRange(0, text.length);

    // 执行复制
    const successful = document.execCommand("copy");

    // 清理
    document.body.removeChild(textArea);

    return successful;
  } catch (error) {
    console.error("传统复制方法失败:", error);
    // 确保清理
    if (document.body.contains(textArea)) {
      document.body.removeChild(textArea);
    }
    return false;
  }
}

/**
 * 为元素临时添加 CSS 类
 * @private
 * @param {HTMLElement} element - 目标元素
 * @param {string} className - 要添加的类名
 * @param {number} duration - 持续时间（毫秒）
 */
function addTemporaryClass(element, className, duration) {
  if (!element || !className) return;

  element.classList.add(className);
  setTimeout(() => {
    element.classList.remove(className);
  }, duration);
}

/**
 * 复制表格单元格内容
 * @param {HTMLElement} cell - 表格单元格元素
 * @param {Object} [options={}] - 复制选项（同 copyToClipboard）
 * @returns {Promise<boolean>} 是否复制成功
 * @example
 * // 在表格单元格点击事件中使用
 * td.onclick = () => copyTableCellContent(td);
 */
async function copyTableCellContent(cell, options = {}) {
  if (!cell) {
    console.error("copyTableCellContent: 未提供有效的单元格元素");
    return false;
  }

  // 获取纯文本内容（去除 HTML 标签）
  const text = cell.textContent || cell.innerText || "";

  return copyToClipboard(text.trim(), {
    ...options,
    sourceElement: cell,
  });
}

/**
 * 创建带复制功能的按钮
 * @param {string} text - 要复制的文本
 * @param {Object} [options={}] - 按钮配置选项
 * @param {string} [options.buttonText='复制'] - 按钮文本
 * @param {string} [options.buttonClass='copy-button'] - 按钮CSS类
 * @param {string} [options.copiedText='已复制'] - 复制成功后的按钮文本
 * @param {number} [options.resetDelay=2000] - 按钮文本重置延迟（毫秒）
 * @returns {HTMLButtonElement} 创建的按钮元素
 * @example
 * // 创建一个复制按钮
 * const copyBtn = createCopyButton('要复制的文本', {
 *   buttonText: '复制密钥',
 *   copiedText: '✓ 已复制'
 * });
 * document.getElementById('container').appendChild(copyBtn);
 */
function createCopyButton(text, options = {}) {
  const {
    buttonText = "复制",
    buttonClass = "copy-button",
    copiedText = "已复制",
    resetDelay = 2000,
  } = options;

  const button = document.createElement("button");
  button.textContent = buttonText;
  button.className = buttonClass;
  button.type = "button";

  button.addEventListener("click", async () => {
    const originalText = button.textContent;

    const success = await copyToClipboard(text, {
      sourceElement: button,
      showMessage: true,
    });

    if (success) {
      button.textContent = copiedText;
      button.disabled = true;

      setTimeout(() => {
        button.textContent = originalText;
        button.disabled = false;
      }, resetDelay);
    }
  });

  return button;
}

/**
 * 检查剪贴板 API 是否可用
 * @returns {boolean} 是否支持 Clipboard API
 * @example
 * if (isClipboardSupported()) {
 *   console.log('浏览器支持现代剪贴板 API');
 * }
 */
function isClipboardSupported() {
  return !!(navigator.clipboard && window.isSecureContext);
}

/**
 * 从剪贴板读取文本（需要用户权限）
 * @param {Object} [options={}] - 配置选项
 * @param {boolean} [options.showMessage=true] - 是否显示结果消息
 * @param {Function} [options.onSuccess] - 读取成功时的回调
 * @param {Function} [options.onError] - 读取失败时的回调
 * @returns {Promise<string|null>} 剪贴板中的文本，失败时返回 null
 * @example
 * const text = await readFromClipboard();
 * if (text) {
 *   console.log('剪贴板内容:', text);
 * }
 */
async function readFromClipboard(options = {}) {
  const { showMessage = true, onSuccess, onError } = options;

  if (!isClipboardSupported()) {
    const error = new Error("浏览器不支持剪贴板 API");
    if (showMessage) {
      showGlobalMessage("浏览器不支持读取剪贴板", true);
    }
    if (onError) {
      onError(error);
    }
    return null;
  }

  try {
    const text = await navigator.clipboard.readText();

    if (onSuccess) {
      onSuccess(text);
    }

    return text;
  } catch (error) {
    console.error("读取剪贴板失败:", error);

    if (showMessage) {
      if (error.name === "NotAllowedError") {
        showGlobalMessage("需要您的许可才能读取剪贴板", true);
      } else {
        showGlobalMessage("无法读取剪贴板内容", true);
      }
    }

    if (onError) {
      onError(error);
    }

    return null;
  }
}
