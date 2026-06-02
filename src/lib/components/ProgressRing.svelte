<script lang="ts">
  interface Props {
    progress: number;
    timeText: string;
    size?: number;
  }

  let { progress, timeText, size = 240 }: Props = $props();

  const stroke = 10;
  const radius = $derived((size - stroke) / 2);
  const circumference = $derived(2 * Math.PI * radius);
  const offset = $derived(circumference * (1 - Math.min(Math.max(progress, 0), 1)));
</script>

<div class="ring-wrap" style={`width:${size}px;height:${size}px`}>
  <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`}>
    <circle
      cx={size / 2}
      cy={size / 2}
      r={radius}
      fill="none"
      stroke="#e8eef0"
      stroke-width={stroke}
    />
    <circle
      cx={size / 2}
      cy={size / 2}
      r={radius}
      fill="none"
      stroke="var(--primary)"
      stroke-width={stroke}
      stroke-linecap="round"
      stroke-dasharray={circumference}
      stroke-dashoffset={offset}
      transform={`rotate(-90 ${size / 2} ${size / 2})`}
    />
  </svg>
  <div class="time">{timeText}</div>
</div>

<style>
  .ring-wrap {
    position: relative;
    margin: 0 auto;
  }

  svg {
    display: block;
  }

  .time {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 52px;
    font-weight: 700;
    color: var(--primary);
    letter-spacing: 1px;
    font-family: "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
  }
</style>
