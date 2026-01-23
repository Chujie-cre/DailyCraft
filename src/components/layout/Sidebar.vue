<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  (e: 'pageChange', page: string): void;
}>();

const currentPage = ref('home');

function handlePageChange(page: string) {
  currentPage.value = page;
  emit('pageChange', page);
}
</script>

<style scoped>
  .radio-input input {
    display: none;
  }

  .radio-input {
    --container_height: 672px;
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    border-radius: 0;
    padding: 10px;
    gap: var(--padding);
    background-color: none;
    color: #000000;
    height: var(--container_height);
    min-width: 110px;
    overflow: hidden;
    border: none;
    border-right: 1px solid rgba(0, 0, 0, 0.1);
  }

  .radio-input label {
    width: 100%;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 4px;
    z-index: 1;
    font-weight: 600;
    font-size: 14px;
    height: 100%;
  }
  .radio-input label svg {
    width: 30px;
    height: 30px;
  }

  .selection {
    display: none;
    position: absolute;
    height: calc(var(--container_height) / 7 - 12px);
    z-index: 0;
    inset: 6px;
    border-radius: 10px;
    transition: 0.15s ease;
  }

  .radio-input label:has(input:checked) {
    color: #fff;
  }

  .radio-input label:has(input:checked) ~ .selection {
    display: inline-block;
  }
  .radio-input label:nth-child(5):has(input:checked) svg {
    stroke: white;
  }

  .radio-input label:nth-child(1):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 0 / 7));
    background-color: #000000;
  }

  .radio-input label:nth-child(2):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 1 / 7));
    background-color: #1f9cf1;
  }

  .radio-input label:nth-child(3):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 2 / 7));
    background-color: #ff9801;
  }

  .radio-input label:nth-child(4):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 3 / 7));
    background-color: #8b5cf6;
  }

  .radio-input label:nth-child(5):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 4 / 7));
    background-color: #6366f1;
  }

  .radio-input label:nth-child(6):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 5 / 7));
    background-color: #e4a0e3;
  }

  .radio-input label:nth-child(7):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 6 / 7));
    background-color: #b7f089;
  }
</style>

<template>
  <div class="radio-input">
    <label @click="handlePageChange('home')">
      <input
        value="value-1"
        :checked="currentPage === 'home'"
        name="value-radio"
        id="value-1"
        type="radio"
      />
      <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 100 100">
        <path fill="#fb7369" d="M91.974,39.775c-0.004-0.004-0.008-0.015-0.012-0.017C79.987,29.693,67.912,20.26,57.173,13.347	c-1.183-0.761-2.431-1.415-3.762-1.857c-0.02-0.007-0.04-0.013-0.06-0.02c-1.073-0.352-2.192-0.529-3.312-0.536v-0.003	c-0.013,0-0.026,0.001-0.039,0.002c-0.013,0-0.026-0.002-0.039-0.002v0.003c-1.12,0.008-2.239,0.185-3.312,0.536	c-0.02,0.007-0.04,0.013-0.06,0.02c-1.331,0.442-2.58,1.095-3.762,1.857C32.088,20.26,20.013,29.693,8.037,39.757	c-0.003,0.003-0.008,0.014-0.012,0.017c-1.803,1.483-2.466,3.788-1.454,5.491c1.144,1.924,3.987,2.344,6.35,0.939	c0.117-0.07,0.215-0.156,0.326-0.232c12.269,0.954,24.506,1.419,36.713,1.428v0.001c0.013,0,0.026,0,0.039,0s0.026,0,0.039,0v-0.001	c12.208-0.01,24.445-0.475,36.713-1.428c0.11,0.076,0.208,0.162,0.326,0.232c2.363,1.405,5.206,0.985,6.35-0.939	C94.441,43.563,93.777,41.257,91.974,39.775z"></path><path fill="#c9dcf4" d="M86.755,45.974l0.008-0.004c-13.426-7.7-25.521-15.401-35.778-21.726c-0.653-0.403-1.481-0.409-2.142-0.018	c-11.11,6.584-22.981,14.07-35.59,21.747c-0.002,0-0.004,0-0.006,0c-0.004,0.003-0.007,0.005-0.011,0.008	c0.865,11.972,2.129,23.603,3.643,34.818c0.803,4.868,4.069,8.202,8.8,8.202h48.641c4.731,0,7.774-3.539,8.8-8.202	c2.118-11.675,3.372-23.263,3.722-34.761C86.814,46.016,86.785,45.995,86.755,45.974z"></path><path fill="#4a254b" d="M50,74.104c3.655,0,6.664-2.76,7.06-6.309C57.108,67.357,56.803,67,56.362,67c-2.45,0-10.274,0-12.724,0	c-0.44,0-0.746,0.357-0.697,0.795C43.335,71.344,46.345,74.104,50,74.104z"></path><circle cx="32.927" cy="59.747" r="6.991" fill="#fff"></circle><circle cx="32.927" cy="59.747" r="3.178" fill="#4a254b"></circle><circle cx="67.073" cy="59.747" r="6.991" fill="#fff"></circle><circle cx="67.073" cy="59.747" r="3.178" fill="#4a254b"></circle><path fill="#fb7369" d="M81,33H69V20.691C69,19.205,70.205,18,71.691,18h6.617C79.795,18,81,19.205,81,20.691V33z"></path>
      </svg>

      <span>首页</span>
    </label>
    <label @click="handlePageChange('logs')">
      <input value="value-2" :checked="currentPage === 'logs'" name="value-radio" id="value-2" type="radio" />
      <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 48 48">
        <path fill="#90a4ae" d="M36.149,25.242L18.579,9.055V7h-3.11c-0.065,0.003-2.192-0.003-2.192,2.182v25.836	c0,1.754,1.523,2.107,2.114,2.109h3.184v-2.08l2.649-2.43l3.9,8.918c0.367,0.92,1.264,1.403,2.153,1.447V43h3v-1.06l1.518-0.682	c0.528-0.234,0.924-0.654,1.116-1.185c0.203-0.559,0.156-1.179-0.117-1.715l-4.06-8.697l6.096-0.571l0.152-0.026	c1.292-0.317,1.699-1.125,1.828-1.579C37.023,26.727,36.774,25.856,36.149,25.242z"></path><path fill="#e0e0e0" stroke="#37474f" stroke-miterlimit="10" stroke-width="2" d="M29.137,41.156l-4.482-10.248l-5.377,4.932	c-0.656,0.559-1.912,0.307-1.912-0.822V9.182c0-1.154,1.325-1.516,2.031-0.822l19.12,17.616c0.576,0.566,0.723,1.76-0.717,2.114	l-7.528,0.705l4.66,9.982c0.33,0.648,0.082,1.311-0.489,1.565l-3.574,1.606C30.298,42.152,29.418,41.884,29.137,41.156z"></path>
      </svg>
      <span>日志面板</span>
    </label>
    <label @click="handlePageChange('diary')">
      <input value="value-diary" :checked="currentPage === 'diary'" name="value-radio" id="value-diary" type="radio" />
      <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 48 48">
        <path fill="#64b5f6" d="M31,14h13v-4c0-2.209-1.791-4-4-4h-9V14z"></path><path fill="#bbdefb" d="M40,6H14c-2.209,0-4,1.791-4,4v28c0,2.209,1.791,4,4,4h18c2.209,0,4-1.791,4-4V10 C36,7.791,37.791,6,40,6z"></path><path fill="#64b5f6" d="M32 42H8c-2.209 0-4-1.791-4-4v-3h24v3C28 40.209 29.791 42 32 42zM16 14H30V16H16zM16 19H26V21H16zM16 24H30V26H16zM16 29H26V31H16z"></path>
      </svg>
      <span>AI日报</span>
    </label>
    <label @click="handlePageChange('screenshots')">
      <input value="value-screenshots" :checked="currentPage === 'screenshots'" name="value-radio" id="value-screenshots" type="radio" />
      <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 48 48">
        <path fill="#37474F" d="M33 10A6 6 0 1 0 33 22 6 6 0 1 0 33 10zM15 10A6 6 0 1 0 15 22 6 6 0 1 0 15 10z"></path><path fill="#37474F" d="M46.7,25l-15.3,3H16.7L1.4,25l4.3-7.9c1.1-1.9,3.1-3.1,5.3-3.1h26.2c2.2,0,4.2,1.2,5.3,3.1L46.7,25z"></path><path fill="#37474F" d="M38 20A10 10 0 1 0 38 40 10 10 0 1 0 38 20zM10 20A10 10 0 1 0 10 40 10 10 0 1 0 10 20z"></path><path fill="#37474F" d="M24 23A5 5 0 1 0 24 33A5 5 0 1 0 24 23Z"></path><path fill="#546E7A" d="M24 26A2 2 0 1 0 24 30A2 2 0 1 0 24 26Z"></path><g><path fill="#A0F" d="M38 23A7 7 0 1 0 38 37 7 7 0 1 0 38 23zM10 23A7 7 0 1 0 10 37 7 7 0 1 0 10 23z"></path></g><g><path fill="#CE93D8" d="M41.7 27.7c-1-1.1-2.3-1.7-3.7-1.7s-2.8.6-3.7 1.7c-.4.4-.3 1 .1 1.4.4.4 1 .3 1.4-.1 1.2-1.3 3.3-1.3 4.5 0 .2.2.5.3.7.3.2 0 .5-.1.7-.3C42.1 28.7 42.1 28.1 41.7 27.7zM10 26c-1.4 0-2.8.6-3.7 1.7-.4.4-.3 1 .1 1.4.4.4 1 .3 1.4-.1 1.2-1.3 3.3-1.3 4.5 0 .2.2.5.3.7.3.2 0 .5-.1.7-.3.4-.4.4-1 .1-1.4C12.8 26.6 11.4 26 10 26z"></path></g>
      </svg>
      <span>记录帧</span>
    </label>
    <label @click="handlePageChange('chat')">
      <input value="value-chat" :checked="currentPage === 'chat'" name="value-radio" id="value-chat" type="radio" />
      <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 48 48">
        <path fill="#8BC34A" d="M37,39H11l-6,6V11c0-3.3,2.7-6,6-6h26c3.3,0,6,2.7,6,6v22C43,36.3,40.3,39,37,39z"></path>
      </svg>
      <span>问问AI</span>
    </label>
    <label @click="handlePageChange('settings')">
      <input value="value-3" :checked="currentPage === 'settings'" name="value-radio" id="value-3" type="radio" />
      <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 48 48">
        <path fill="#607d8b" d="M39.6,27.2c0.1-0.7,0.2-1.4,0.2-2.2s-0.1-1.5-0.2-2.2l4.5-3.2c0.4-0.3,0.6-0.9,0.3-1.4L40,10.8	c-0.3-0.5-0.8-0.7-1.3-0.4l-5,2.3c-1.2-0.9-2.4-1.6-3.8-2.2L29.4,5c-0.1-0.5-0.5-0.9-1-0.9h-8.6c-0.5,0-1,0.4-1,0.9l-0.5,5.5	c-1.4,0.6-2.7,1.3-3.8,2.2l-5-2.3c-0.5-0.2-1.1,0-1.3,0.4l-4.3,7.4c-0.3,0.5-0.1,1.1,0.3,1.4l4.5,3.2c-0.1,0.7-0.2,1.4-0.2,2.2	s0.1,1.5,0.2,2.2L4,30.4c-0.4,0.3-0.6,0.9-0.3,1.4L8,39.2c0.3,0.5,0.8,0.7,1.3,0.4l5-2.3c1.2,0.9,2.4,1.6,3.8,2.2l0.5,5.5	c0.1,0.5,0.5,0.9,1,0.9h8.6c0.5,0,1-0.4,1-0.9l0.5-5.5c1.4-0.6,2.7-1.3,3.8-2.2l5,2.3c0.5,0.2,1.1,0,1.3-0.4l4.3-7.4	c0.3-0.5,0.1-1.1-0.3-1.4L39.6,27.2z M24,35c-5.5,0-10-4.5-10-10s4.5-10,10-10s10,4.5,10,10S29.5,35,24,35z"></path><path fill="#455a64" d="M24,13c-6.6,0-12,5.4-12,12s5.4,12,12,12s12-5.4,12-12S30.6,13,24,13z M24,30c-2.8,0-5-2.2-5-5	s2.2-5,5-5s5,2.2,5,5S26.8,30,24,30z"></path>
      </svg>
      <span>设置</span>
    </label>
    <label @click="handlePageChange('about')">
      <input value="value-4" :checked="currentPage === 'about'" name="value-radio" id="value-4" type="radio" />
      <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="100" height="100" viewBox="0 0 48 48">
        <linearGradient id="-KrjULclfATmpOK5cCvRJa_RwIDIrubP0Fk_gr1" x1="23.746" x2="27.162" y1="7.643" y2="40.873" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="#f189ae"></stop><stop offset="1" stop-color="#ee4c95"></stop></linearGradient><path fill="url(#-KrjULclfATmpOK5cCvRJa_RwIDIrubP0Fk_gr1)" fill-rule="evenodd" d="M40.277,5.542c-0.051,0-0.103,0-0.153,0	c-4.966,0-12.081,2.076-14.541,3.207l-0.391,0.178c-0.339,0.154-0.803,0.366-1.156,0.529C23.67,8.694,23.246,7.995,22.8,7.4	c-1.463-1.951-3.675-4.9-9.289-4.9c-1.585,0-4.002,0.861-5.599,3.287c-3.877,5.886-2.011,18.64-1.903,19.346	c-0.133,0.073-0.254,0.167-0.346,0.3c-2.377,3.448-3.156,7.322-2.195,10.908c1.668,6.222,8.194,10.152,15.779,10.152	c1.934,0,3.938-0.255,5.946-0.794c4.742-1.271,8.825-3.914,11.496-7.443c2.729-3.604,3.666-7.697,2.64-11.524	c-0.873-3.261-3.132-5.963-6.262-7.761c0.628,0.014,1.26,0.025,1.891,0.025c2.295,0,4.549-0.114,6.258-0.494	c5.467-1.215,6.283-4.882,6.283-6.96C47.5,9.156,45.576,5.589,40.277,5.542z" clip-rule="evenodd"></path><ellipse cx="22.94" cy="37.175" fill="none" stroke="#1d1d1b" stroke-miterlimit="10" opacity=".07" rx="13.794" ry="7.656" transform="rotate(-14.999 22.941 37.178)"></ellipse><path fill="#1d1d1b" d="M26.935,28.962c4.783,0,8.543,1.71,9.329,4.642c1.094,4.081-3.989,8.995-11.342,10.965 c-2.07,0.555-4.102,0.818-5.976,0.818c-4.783,0-8.543-1.71-9.329-4.643c-1.094-4.082,3.989-8.995,11.342-10.965 C23.029,29.225,25.06,28.962,26.935,28.962 M26.935,27.962L26.935,27.962c-2.029,0-4.127,0.287-6.236,0.852 c-3.718,0.996-7.012,2.75-9.275,4.938c-2.416,2.336-3.402,4.911-2.774,7.252c0.89,3.321,4.835,5.384,10.295,5.384 c2.028,0,4.126-0.287,6.235-0.852c3.718-0.996,7.012-2.75,9.275-4.938c2.416-2.336,3.401-4.911,2.774-7.252 C36.34,30.025,32.395,27.962,26.935,27.962L26.935,27.962z" opacity=".05"></path><ellipse cx="22.94" cy="37.175" fill="#fff" rx="13.794" ry="7.656" transform="rotate(-14.999 22.941 37.178)"></ellipse><ellipse cx="15.5" cy="38.5" fill="none" stroke="#1d1d1b" stroke-miterlimit="10" opacity=".07" rx="1.5" ry="2.5"></ellipse><path fill="#1d1d1b" d="M15.5,36c0.828,0,1.5,1.12,1.5,2.5S16.328,41,15.5,41S14,39.88,14,38.5S14.672,36,15.5,36 M15.5,35 c-1.425,0-2.5,1.505-2.5,3.5s1.075,3.5,2.5,3.5s2.5-1.505,2.5-3.5S16.925,35,15.5,35L15.5,35z" opacity=".05"></path><ellipse cx="15.5" cy="38.5" rx="1.5" ry="2.5"></ellipse><ellipse cx="31.5" cy="35.5" fill="none" stroke="#1d1d1b" stroke-miterlimit="10" opacity=".07" rx="1.5" ry="2.5"></ellipse><path fill="#1d1d1b" d="M31.5,33c0.828,0,1.5,1.12,1.5,2.5S32.328,38,31.5,38S30,36.88,30,35.5S30.672,33,31.5,33 M31.5,32 c-1.425,0-2.5,1.505-2.5,3.5s1.075,3.5,2.5,3.5s2.5-1.505,2.5-3.5S32.925,32,31.5,32L31.5,32z" opacity=".05"></path><ellipse cx="31.5" cy="35.5" rx="1.5" ry="2.5"></ellipse><path fill="#ffeb00" d="M24.153,38.804c1.025-0.18,1.955,0.229,2.075,0.913c0.12,0.683-0.615,1.384-1.641,1.564 s-1.955-0.229-2.075-0.913C22.392,39.684,23.128,38.984,24.153,38.804z"></path><path fill="none" stroke="#1d1d1b" stroke-miterlimit="10" d="M5.575,22.005C5.833,20.858,6.858,20,8.083,20	c1.418,0,2.57,1.152,2.57,2.57c0,0.642-0.236,1.229-0.625,1.679c1.131,0.27,1.972,1.287,1.972,2.5c0,1.418-1.152,2.57-2.57,2.57	c-0.697,0-1.329-0.278-1.792-0.728c-0.309,1.149-1.359,1.996-2.605,1.996c-1.489,0-2.698-1.209-2.698-2.698	c0-0.528,0.152-1.021,0.415-1.437c-1.035-0.377-1.776-1.371-1.776-2.536c0-1.489,1.209-2.698,2.698-2.698	C4.414,21.219,5.087,21.519,5.575,22.005z" opacity=".07"></path><g opacity=".05"><path fill="#1d1d1b" d="M8.083,20c1.418,0,2.57,1.152,2.57,2.57c0,0.642-0.236,1.229-0.625,1.679 c1.131,0.27,1.973,1.287,1.973,2.5c0,1.418-1.152,2.57-2.57,2.57c-0.697,0-1.329-0.278-1.792-0.728 c-0.309,1.149-1.359,1.996-2.605,1.996c-1.489,0-2.698-1.209-2.698-2.698c0-0.528,0.152-1.021,0.415-1.437 c-1.035-0.377-1.776-1.371-1.776-2.536c0-1.489,1.209-2.698,2.698-2.698c0.742,0,1.415,0.301,1.903,0.786 C5.833,20.858,6.858,20,8.083,20 M8.083,19c-1.183,0-2.269,0.598-2.925,1.529c-0.463-0.203-0.966-0.31-1.486-0.31 c-2.039,0-3.698,1.659-3.698,3.698c0,1.187,0.575,2.285,1.499,2.972c-0.091,0.324-0.138,0.66-0.138,1.001 c0,2.039,1.659,3.698,3.698,3.698c1.217,0,2.328-0.598,3.008-1.547c0.434,0.184,0.905,0.28,1.389,0.28 c1.969,0,3.57-1.602,3.57-3.57c0-1.216-0.605-2.308-1.564-2.956c0.142-0.389,0.216-0.803,0.216-1.223 C11.653,20.601,10.051,19,8.083,19L8.083,19z"></path></g><path fill="#fff" d="M5.575,22.005C5.833,20.858,6.858,20,8.083,20c1.418,0,2.57,1.152,2.57,2.57	c0,0.642-0.236,1.229-0.625,1.679c1.131,0.27,1.972,1.287,1.972,2.5c0,1.418-1.152,2.57-2.57,2.57c-0.697,0-1.329-0.278-1.792-0.728	c-0.309,1.149-1.359,1.996-2.605,1.996c-1.489,0-2.698-1.209-2.698-2.698c0-0.528,0.152-1.021,0.415-1.437	c-1.035-0.377-1.776-1.371-1.776-2.536c0-1.489,1.209-2.698,2.698-2.698C4.414,21.219,5.087,21.519,5.575,22.005z"></path><path fill="#ffeb00" fill-rule="evenodd" d="M5.989,24.058c0.808-0.217,1.621,0.192,1.814,0.911	c0.193,0.719-0.307,1.48-1.115,1.696s-1.621-0.192-1.814-0.911C4.68,25.035,5.18,24.275,5.989,24.058z" clip-rule="evenodd"></path><path fill="#e24880" fill-rule="evenodd" d="M11.265,13.079c0,0,4.116,7.791,9.158,6.075	c6.061-1.519,3.739-9.456,3.613-9.699c0.109-0.052,0.292-0.134,0.396-0.183c0,0,4.563,9.818-3.661,11.714	C15.557,22.187,12.846,18.396,11.265,13.079z" clip-rule="evenodd"></path><path fill="#e24880" fill-rule="evenodd" d="M33.121,18.981c-1.398-0.799-3.245-1.695-5.398-2.556	c2.914,0.336,5.616,1.323,8.402,2.556H33.121z" clip-rule="evenodd"></path>
      </svg>
      <span>关于</span>
    </label>
    <span class="selection"></span>
  </div>
</template>
