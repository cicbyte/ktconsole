<template>
  <a-modal
    :open="open"
    @cancel="handleClose"
    :footer="null"
    :width="520"
    :destroy-on-close="true"
    :closable="true"
    class="create-tunnel-modal"
  >
    <!-- Header -->
    <div class="modal-header">
      <h3 class="modal-title">新建隧道</h3>
      <p class="modal-subtitle">配置并启动一个新的隧道连接</p>
    </div>

    <!-- Steps Indicator -->
    <div class="steps-bar">
      <div
        v-for="(step, idx) in steps"
        :key="idx"
        class="step-node"
        :class="{ active: currentStep === idx, done: currentStep > idx }"
      >
        <div class="step-dot">
          <CheckOutlined v-if="currentStep > idx" style="font-size: 10px" />
          <span v-else>{{ idx + 1 }}</span>
        </div>
        <span class="step-text">{{ step.label }}</span>
        <div v-if="idx < steps.length - 1" class="step-line" :class="{ filled: currentStep > idx }"></div>
      </div>
    </div>

    <!-- Step 1: Basic Info -->
    <div v-if="currentStep === 0" class="step-content">
      <div class="form-field">
        <label class="field-label">隧道名称</label>
        <a-input
          v-model:value="form.name"
          placeholder="留空自动生成"
          allow-clear
        />
      </div>
      <div class="form-field">
        <label class="field-label">服务提供商</label>
        <div class="provider-cards">
          <div
            v-for="provider in availableProviders"
            :key="provider.id"
            class="provider-card"
            :class="{ selected: form.provider === provider.id, disabled: !provider.installed }"
            @click="provider.installed && (form.provider = provider.id)"
          >
            <div class="card-avatar" :style="{ background: providerColor(provider.id) }">
              {{ provider.name[0].toUpperCase() }}
            </div>
            <div class="card-info">
              <span class="card-name">{{ provider.name }}</span>
              <span class="card-status">{{ provider.installed ? '已安装' : '未安装' }}</span>
            </div>
            <div v-if="form.provider === provider.id" class="card-check">
              <CheckOutlined />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Step 2: Protocol Config -->
    <div v-if="currentStep === 1" class="step-content">
      <div class="form-field">
        <label class="field-label">协议类型</label>
        <div class="protocol-pills">
          <div
            v-for="proto in protocols"
            :key="proto"
            class="proto-pill"
            :class="{ active: form.protocol === proto }"
            @click="form.protocol = proto"
          >
            {{ proto }}
          </div>
        </div>
      </div>
      <div class="form-row">
        <div class="form-field" style="flex: 1">
          <label class="field-label">本地端口</label>
          <a-input-number
            v-model:value="form.localPort"
            :min="1"
            :max="65535"
            placeholder="8080"
            style="width: 100%"
          />
        </div>
        <div v-if="form.provider !== 'gradio'" class="form-field" style="flex: 1">
          <label class="field-label">本地地址</label>
          <a-input v-model:value="form.localHost" placeholder="localhost" />
        </div>
      </div>
      <div v-if="form.provider !== 'gradio'" class="form-field">
        <label class="field-label">自定义域名 <span class="hint">可选</span></label>
        <a-input v-model:value="form.customDomain" placeholder="example.com" allow-clear />
      </div>
    </div>

    <!-- Step 3: Advanced Options -->
    <div v-if="currentStep === 2" class="step-content">
      <div class="section-block">
        <div class="section-title">安全与访问</div>
        <div class="toggle-row">
          <div class="toggle-info">
            <span class="toggle-label">基本认证</span>
            <span class="toggle-desc">使用用户名密码保护隧道</span>
          </div>
          <a-switch v-model:checked="form.basicAuth" size="small" />
        </div>
        <div v-if="form.basicAuth" class="auth-fields">
          <div class="form-row">
            <div class="form-field" style="flex: 1">
              <label class="field-label">用户名</label>
              <a-input v-model:value="form.basicAuthUser" placeholder="admin" />
            </div>
            <div class="form-field" style="flex: 1">
              <label class="field-label">密码</label>
              <a-input-password v-model:value="form.basicAuthPass" placeholder="输入密码" />
            </div>
          </div>
        </div>
        <div class="toggle-row">
          <div class="toggle-info">
            <span class="toggle-label">IP 白名单</span>
            <span class="toggle-desc">限制特定 IP 地址访问</span>
          </div>
          <a-switch v-model:checked="form.ipWhitelist" size="small" />
        </div>
      </div>
      <div class="section-block">
        <div class="section-title">性能与调试</div>
        <div class="toggle-row">
          <div class="toggle-info">
            <span class="toggle-label">压缩传输</span>
            <span class="toggle-desc">启用 Gzip 压缩 HTTP 响应</span>
          </div>
          <a-switch v-model:checked="form.compression" size="small" />
        </div>
        <div class="toggle-row">
          <div class="toggle-info">
            <span class="toggle-label">请求检查</span>
            <span class="toggle-desc">启用 HTTP 请求检查面板</span>
          </div>
          <a-switch v-model:checked="form.inspect" size="small" />
        </div>
      </div>
    </div>

    <!-- Footer Actions -->
    <div class="modal-footer">
      <a-button v-if="currentStep > 0" @click="currentStep--">
        <template #icon><LeftOutlined /></template>
        上一步
      </a-button>
      <div v-else></div>
      <div class="footer-right">
        <a-button @click="handleClose">取消</a-button>
        <a-button
          v-if="currentStep < steps.length - 1"
          type="primary"
          @click="nextStep"
          :disabled="!canNext"
        >
          下一步
          <template #icon><RightOutlined /></template>
        </a-button>
        <a-button
          v-if="currentStep === steps.length - 1"
          type="primary"
          @click="handleSubmit"
          :loading="creating"
        >
          <template #icon><ThunderboltOutlined /></template>
          创建隧道
        </a-button>
      </div>
    </div>
  </a-modal>
</template>

<script setup>
import { ref, reactive, computed, watch } from 'vue'
import { message } from 'ant-design-vue'
import {
  CheckOutlined,
  LeftOutlined,
  RightOutlined,
  ThunderboltOutlined,
} from '@ant-design/icons-vue'
import { tunnelService } from '@/services/database'

const props = defineProps({
  open: { type: Boolean, default: false }
})
const emit = defineEmits(['update:open', 'created'])

const currentStep = ref(0)
const creating = ref(false)
const availableProviders = ref([])

const steps = computed(() => {
  const base = [{ label: '基本信息' }, { label: '协议配置' }]
  if (form.provider === 'gradio') return base
  return [...base, { label: '高级选项' }]
})

const allProtocols = ['HTTP', 'HTTPS', 'TCP', 'UDP', 'TLS']
const protocols = computed(() => {
  if (form.provider === 'gradio') return ['HTTP']
  return allProtocols
})

const providerColor = (id) => {
  const colors = {
    ngrok: 'rgba(79, 172, 254, 0.2)',
    cpolar: 'rgba(0, 212, 170, 0.2)',
    frp: 'rgba(255, 217, 61, 0.2)',
    gradio: 'rgba(255, 152, 0, 0.2)',
  }
  return colors[id] || 'var(--color-primary-dim)'
}

const form = reactive({
  name: '',
  provider: '',
  protocol: 'HTTP',
  localPort: 8080,
  localHost: 'localhost',
  customDomain: '',
  basicAuth: false,
  basicAuthUser: '',
  basicAuthPass: '',
  ipWhitelist: false,
  compression: false,
  inspect: false,
})

const canNext = computed(() => {
  if (currentStep.value === 0) return !!form.provider
  if (currentStep.value === 1) return !!form.localPort && !!form.localHost
  return true
})

const nextStep = () => {
  if (canNext.value && currentStep.value < steps.value.length - 1) {
    currentStep.value++
  }
}

const handleClose = () => {
  currentStep.value = 0
  Object.assign(form, {
    name: '', provider: '', protocol: 'HTTP', localPort: 8080,
    localHost: 'localhost', customDomain: '', basicAuth: false,
    basicAuthUser: '', basicAuthPass: '',
    ipWhitelist: false, compression: false, inspect: false,
  })
  emit('update:open', false)
}

const handleSubmit = async () => {
  creating.value = true
  try {
    await tunnelService.create({
      name: form.name || undefined,
      provider: form.provider,
      protocol: form.protocol,
      localPort: String(form.localPort),
      localHost: form.localHost,
      customDomain: form.customDomain || undefined,
      basicAuth: form.basicAuth,
      basicAuthUser: form.basicAuthUser || undefined,
      basicAuthPass: form.basicAuthPass || undefined,
      ipWhitelist: form.ipWhitelist,
      compression: form.compression,
      inspect: form.inspect,
    })
    message.success('隧道创建成功')
    emit('created')
    handleClose()
  } catch (error) {
    message.error('创建失败: ' + error)
  } finally {
    creating.value = false
  }
}

const loadProviders = async () => {
  try {
    availableProviders.value = await tunnelService.detectProviders()
  } catch {
    availableProviders.value = [
      { id: 'ngrok', name: 'ngrok', installed: false, status: 'not_installed' },
      { id: 'cpolar', name: 'cpolar', installed: false, status: 'not_installed' },
      { id: 'frp', name: 'FRP', installed: false, status: 'not_installed' },
      { id: 'gradio', name: 'Gradio', installed: false, status: 'not_installed' },
    ]
  }
}

watch(() => props.open, (val) => {
  if (val) loadProviders()
})
</script>

<style scoped>
/* Header */
.modal-header {
  margin-bottom: 20px;
}

.modal-title {
  margin: 0 0 4px;
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text);
}

.modal-subtitle {
  margin: 0;
  font-size: 0.8rem;
  color: var(--color-text-dim);
}

/* Steps Bar */
.steps-bar {
  display: flex;
  align-items: center;
  margin-bottom: 24px;
  padding: 12px 16px;
  background: var(--color-surface-alt);
  border-radius: var(--border-radius-md);
}

.step-node {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  position: relative;
}

.step-dot {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.7rem;
  font-weight: 700;
  border: 2px solid var(--color-border-light);
  color: var(--color-text-dim);
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.step-node.active .step-dot {
  border-color: var(--color-primary);
  background: var(--color-primary-dim);
  color: var(--color-primary);
}

.step-node.done .step-dot {
  border-color: var(--color-primary);
  background: var(--color-primary);
  color: var(--color-background);
}

.step-text {
  font-size: 0.75rem;
  color: var(--color-text-dim);
  white-space: nowrap;
  transition: color 0.3s ease;
}

.step-node.active .step-text {
  color: var(--color-primary);
  font-weight: 500;
}

.step-node.done .step-text {
  color: var(--color-text-secondary);
}

.step-line {
  flex: 1;
  height: 2px;
  background: var(--color-border);
  margin: 0 8px;
  border-radius: 1px;
  transition: background 0.3s ease;
}

.step-line.filled {
  background: var(--color-primary);
}

/* Form */
.step-content {
  min-height: 220px;
}

.form-field {
  margin-bottom: 16px;
}

.form-row {
  display: flex;
  gap: 12px;
}

.field-label {
  display: block;
  margin-bottom: 6px;
  font-size: 0.8rem;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.hint {
  color: var(--color-text-dim);
  font-weight: 400;
  font-size: 0.75rem;
}

/* Provider Cards */
.provider-cards {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.provider-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
}

.provider-card:hover:not(.disabled) {
  border-color: var(--color-primary);
  background: var(--color-hover);
}

.provider-card.selected {
  border-color: var(--color-primary);
  background: var(--color-primary-dim);
}

.provider-card.disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.card-avatar {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
  font-weight: 700;
  color: var(--color-text);
  flex-shrink: 0;
}

.card-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
  flex: 1;
}

.card-name {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--color-text);
}

.card-status {
  font-size: 0.7rem;
  color: var(--color-text-dim);
}

.provider-card.selected .card-status {
  color: var(--color-primary);
}

.card-check {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--color-primary);
  color: var(--color-background);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  flex-shrink: 0;
}

/* Protocol Pills */
.protocol-pills {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.proto-pill {
  padding: 6px 16px;
  border-radius: 20px;
  font-size: 0.78rem;
  font-weight: 500;
  cursor: pointer;
  border: 1px solid var(--color-border);
  color: var(--color-text-secondary);
  transition: all 0.2s ease;
  user-select: none;
}

.proto-pill:hover {
  border-color: var(--color-primary);
  color: var(--color-text);
}

.proto-pill.active {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--color-background);
  font-weight: 600;
}

/* Section Blocks */
.section-block {
  margin-bottom: 16px;
  padding: 12px;
  background: var(--color-surface-alt);
  border-radius: var(--border-radius-md);
}

.section-block:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--color-text-dim);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 10px;
}

.toggle-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 0;
}

.toggle-row + .toggle-row {
  border-top: 1px solid var(--color-border);
  padding-top: 10px;
  margin-top: 4px;
}

.toggle-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.toggle-label {
  font-size: 0.82rem;
  color: var(--color-text);
}

.toggle-desc {
  font-size: 0.7rem;
  color: var(--color-text-dim);
}

/* Footer */
.modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--color-border);
}

.footer-right {
  display: flex;
  gap: 8px;
}

/* Auth Fields */
.auth-fields {
  margin-top: 8px;
  padding: 8px 12px;
  background: var(--color-surface-alt);
  border-radius: var(--border-radius-md);
}
</style>
