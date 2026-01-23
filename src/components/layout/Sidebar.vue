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
    --container_height: 576px;
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
    height: calc(var(--container_height) / 6 - 12px);
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
    transform: translateY(calc(var(--container_height) * 0 / 6));
    background-color: #000000;
  }

  .radio-input label:nth-child(2):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 1 / 6));
    background-color: #1f9cf1;
  }

  .radio-input label:nth-child(3):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 2 / 6));
    background-color: #ff9801;
  }

  .radio-input label:nth-child(4):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 3 / 6));
    background-color: #8b5cf6;
  }

  .radio-input label:nth-child(5):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 4 / 6));
    background-color: #e4a0e3;
  }

  .radio-input label:nth-child(6):has(input:checked) ~ .selection {
    transform: translateY(calc(var(--container_height) * 5 / 6));
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
      <svg
        xmlns="http://www.w3.org/2000/svg"
        preserveAspectRatio="xMidYMid"
        viewBox="0 0 256 256"
      >
        <defs>
          <linearGradient x1="37%" y1="51%" x2="178.1%" y2="41.9%" id="a">
            <stop stop-color="#FC801D" offset="9%"></stop>
            <stop stop-color="#B07F61" offset="23%"></stop>
            <stop stop-color="#577DB3" offset="41%"></stop>
            <stop stop-color="#1E7CE6" offset="53%"></stop>
            <stop stop-color="#087CFA" offset="59%"></stop>
          </linearGradient>
          <linearGradient x1="73.6%" y1="114.8%" x2="35.6%" y2="1.1%" id="b">
            <stop stop-color="#FE2857" offset="0%"></stop>
            <stop stop-color="#CB3979" offset="8%"></stop>
            <stop stop-color="#9E4997" offset="16%"></stop>
            <stop stop-color="#7557B2" offset="25%"></stop>
            <stop stop-color="#5362C8" offset="34%"></stop>
            <stop stop-color="#386CDA" offset="44%"></stop>
            <stop stop-color="#2373E8" offset="54%"></stop>
            <stop stop-color="#1478F2" offset="66%"></stop>
            <stop stop-color="#0B7BF8" offset="79%"></stop>
            <stop stop-color="#087CFA" offset="100%"></stop>
          </linearGradient>
          <linearGradient x1="28.6%" y1="23.6%" x2="81.8%" y2="129.8%" id="c">
            <stop stop-color="#FE2857" offset="0%"></stop>
            <stop stop-color="#FE295F" offset="8%"></stop>
            <stop stop-color="#FF2D76" offset="21%"></stop>
            <stop stop-color="#FF318C" offset="30%"></stop>
            <stop stop-color="#EA3896" offset="38%"></stop>
            <stop stop-color="#B248AE" offset="55%"></stop>
            <stop stop-color="#5A63D6" offset="79%"></stop>
            <stop stop-color="#087CFA" offset="100%"></stop>
          </linearGradient>
        </defs>
        <path fill="url(#a)" d="M40.5 180.6 2.9 150.8l22.1-41 33.3 11.1z"></path>
        <path fill="#087CFA" d="m256 68.2-4.6 148.3-98.6 39.5-53.7-34.7z"></path>
        <path fill="url(#b)" d="m256 68.2-48.8 47.6L144.5 39l31-34.8z"></path>
        <path
          fill="url(#c)"
          d="m99.1 221.3-78.5 28.4 16.5-57.5 21.2-71.3L0 101.4 37.1 0l83.8 9.9 86.3 105.9z"
        ></path>
        <path d="M49.1 48h160v160h-160z"></path>
        <path
          d="M69 177.8h60v10H69v-10ZM99 79V68H69.2v11h8.4v37.7h-8.4v11H99v-11h-8.3V79H99Zm28.5 49.4.2.1c-4.1.2-8.1-.8-11.8-2.6a27 27 0 0 1-7.7-6.3l8.2-9.2c1.5 1.7 3.2 3.1 5.2 4.3 1.7 1.1 3.7 1.7 5.7 1.6 2.2.2 4.3-.7 5.8-2.3a11 11 0 0 0 2.2-7.5V68h13.3v39a27 27 0 0 1-1.5 9.4c-1.7 5-5.7 9-10.8 10.6-2.8 1-5.8 1.5-8.8 1.4Z"
          fill="#FFF"
        ></path>
      </svg>

      <span>首页</span>
    </label>
    <label @click="handlePageChange('logs')">
      <input value="value-2" :checked="currentPage === 'logs'" name="value-radio" id="value-2" type="radio" />
      <svg
        viewBox="0 0 256 254"
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        width="256"
        height="254"
        preserveAspectRatio="xMidYMid"
      >
        <defs>
          <linearGradient id="c" x1="50%" x2="50%" y1="0%" y2="100%">
            <stop offset="0%" stop-color="#FFF"></stop>
            <stop offset="100%" stop-color="#FFF" stop-opacity="0"></stop>
          </linearGradient>
          <path
            id="a"
            d="M180.828 252.605a15.872 15.872 0 0 0 12.65-.486l52.501-25.262a15.94 15.94 0 0 0 9.025-14.364V41.197a15.939 15.939 0 0 0-9.025-14.363l-52.5-25.263a15.877 15.877 0 0 0-18.115 3.084L74.857 96.35l-43.78-33.232a10.614 10.614 0 0 0-13.56.603L3.476 76.494c-4.63 4.211-4.635 11.495-.012 15.713l37.967 34.638-37.967 34.637c-4.623 4.219-4.618 11.502.012 15.714l14.041 12.772a10.614 10.614 0 0 0 13.56.604l43.78-33.233 100.507 91.695a15.853 15.853 0 0 0 5.464 3.571Zm10.464-183.649-76.262 57.889 76.262 57.888V68.956Z"
          ></path>
        </defs>
        <mask id="b" fill="#fff"></mask>
        <path
          fill="#0065A9"
          d="M246.135 26.873 193.593 1.575a15.885 15.885 0 0 0-18.123 3.08L3.466 161.482c-4.626 4.219-4.62 11.502.012 15.714l14.05 12.772a10.625 10.625 0 0 0 13.569.604L238.229 33.436c6.949-5.271 16.93-.315 16.93 8.407v-.61a15.938 15.938 0 0 0-9.024-14.36Z"
        ></path>
        <path
          fill="#007ACC"
          d="m246.135 226.816-52.542 25.298a15.887 15.887 0 0 1-18.123-3.08L3.466 92.207c-4.626-4.218-4.62-11.502.012-15.713l14.05-12.773a10.625 10.625 0 0 1 13.569-.603l207.132 157.135c6.949 5.271 16.93.315 16.93-8.408v.611a15.939 15.939 0 0 1-9.024 14.36Z"
        ></path>
        <path
          fill="#1F9CF0"
          d="M193.428 252.134a15.892 15.892 0 0 1-18.125-3.083c5.881 5.88 15.938 1.715 15.938-6.603V11.273c0-8.318-10.057-12.483-15.938-6.602a15.892 15.892 0 0 1 18.125-3.084l52.533 25.263a15.937 15.937 0 0 1 9.03 14.363V212.51c0 6.125-3.51 11.709-9.03 14.363l-52.533 25.262Z"
        ></path>
        <path
          fill="url(#c)"
          fill-opacity="0"
          d="M180.828 252.605a15.874 15.874 0 0 0 12.65-.486l52.5-25.263a15.938 15.938 0 0 0 9.026-14.363V41.197a15.939 15.939 0 0 0-9.025-14.363L193.477 1.57a15.877 15.877 0 0 0-18.114 3.084L74.857 96.35l-43.78-33.232a10.614 10.614 0 0 0-13.56.603L3.476 76.494c-4.63 4.211-4.635 11.495-.012 15.713l37.967 34.638-37.967 34.637c-4.623 4.219-4.618 11.502.012 15.714l14.041 12.772a10.614 10.614 0 0 0 13.56.604l43.78-33.233 100.506 91.695a15.857 15.857 0 0 0 5.465 3.571Zm10.464-183.65-76.262 57.89 76.262 57.888V68.956Z"
        ></path>
      </svg>

      <span>日志</span>
    </label>
    <label @click="handlePageChange('diary')">
      <input value="value-diary" :checked="currentPage === 'diary'" name="value-radio" id="value-diary" type="radio" />
      <svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 20h9"></path>
        <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path>
      </svg>
      <span>日记</span>
    </label>
    <label @click="handlePageChange('screenshots')">
      <input value="value-screenshots" :checked="currentPage === 'screenshots'" name="value-radio" id="value-screenshots" type="radio" />
      <svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        <circle cx="8.5" cy="8.5" r="1.5"></circle>
        <polyline points="21 15 16 10 5 21"></polyline>
      </svg>
      <span>截图</span>
    </label>
    <label @click="handlePageChange('settings')">
      <input value="value-3" :checked="currentPage === 'settings'" name="value-radio" id="value-3" type="radio" />
      <svg
        preserveAspectRatio="xMidYMid"
        xmlns="http://www.w3.org/2000/svg"
        height="332"
        width="256"
        viewBox="0 0 256 332"
      >
        <defs>
          <linearGradient
            id="a"
            y2="39.597%"
            x2="63.68%"
            y1="58.68%"
            x1="55.117%"
          >
            <stop offset="0%" stop-color="#FF9700"></stop>
            <stop offset="53%" stop-color="#F48E00"></stop>
            <stop offset="100%" stop-color="#D06F00"></stop>
          </linearGradient>
        </defs>
        <path
          fill="url(#a)"
          d="M255.288 166.795c0-3.887-2.872-6.128-6.397-5.015L6.397 238.675C2.865 239.796 0 243.86 0 247.74v78.59c0 3.887 2.865 6.135 6.397 5.015l242.494-76.888c3.525-1.12 6.397-5.185 6.397-9.071v-78.59Z"
        ></path>
        <path
          fill="#FF9800"
          d="M0 164.291c0 3.887 2.865 7.95 6.397 9.071l242.53 76.902c3.531 1.12 6.397-1.127 6.397-5.007V166.66c0-3.88-2.866-7.944-6.397-9.064L6.397 80.694C2.865 79.574 0 81.814 0 85.7v78.59Z"
        ></path>
        <path
          fill="#FF9800"
          d="M255.288 5.302c0-3.886-2.872-6.135-6.397-5.014L6.397 77.176C2.865 78.296 0 82.36 0 86.247v78.59c0 3.887 2.865 6.128 6.397 5.014l242.494-76.895c3.525-1.12 6.397-5.184 6.397-9.064V5.302Z"
        ></path>
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
