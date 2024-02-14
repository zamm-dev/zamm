<script lang="ts">
  import IconSettings from "~icons/ion/settings";
  import IconChat from "~icons/ph/chat-dots-fill";
  import IconDashboard from "~icons/material-symbols/monitor-heart";
  import IconHeartFill from "~icons/ph/heart-fill";
  import { playSoundEffect } from "$lib/sound";
  import { animationSpeed } from "$lib/preferences";

  const routes: App.Route[] = [
    {
      name: "Dashboard",
      path: "/",
      icon: IconDashboard,
    },
    {
      name: "Chat",
      path: "/chat",
      icon: IconChat,
    },
    {
      name: "Settings",
      path: "/settings",
      icon: IconSettings,
    },
    {
      name: "Credits",
      path: "/credits",
      icon: IconHeartFill,
    },
  ];

  export let currentRoute: string;
  export let dummyLinks = false;
  let indicatorPosition: string;

  function setIndicatorPosition(newRoute: string) {
    const routeIndex = routes.findIndex((r) => r.path === newRoute);
    indicatorPosition = `calc(${routeIndex} * var(--sidebar-icon-size))`;
    return indicatorPosition;
  }

  function playWhooshSound() {
    playSoundEffect("Whoosh", $animationSpeed);
  }

  function routeChangeSimulator(newRoute: App.Route) {
    return (e: MouseEvent) => {
      if (newRoute.path !== currentRoute) {
        playWhooshSound();
      }
      if (dummyLinks) {
        e.preventDefault();
        currentRoute = newRoute.path;
      }
    };
  }

  $: indicatorPosition = setIndicatorPosition(currentRoute);
</script>

<header>
  <svg
    version="1.1"
    style="visibility: hidden; position: absolute;"
    width="0"
    height="0"
  >
    <filter id="inset-shadow">
      <feOffset dx="0" dy="0" />
      <feGaussianBlur stdDeviation="1" result="offset-blur" />
      <feComposite
        operator="out"
        in="SourceGraphic"
        in2="offset-blur"
        result="inverse"
      />
      <feFlood flood-color="#555" flood-opacity=".95" result="color" />
      <feComposite operator="in" in="color" in2="inverse" result="shadow" />
      <feComposite operator="over" in="shadow" in2="SourceGraphic" />
    </filter>

    <filter id="inset-shadow-selected">
      <feOffset dx="0" dy="0" />
      <feGaussianBlur stdDeviation="2" result="offset-blur" />
      <feComposite
        operator="out"
        in="SourceGraphic"
        in2="offset-blur"
        result="inverse"
      />
      <feFlood flood-color="#002966" flood-opacity=".95" result="color" />
      <feComposite operator="in" in="color" in2="inverse" result="shadow" />
      <feComposite operator="over" in="shadow" in2="SourceGraphic" />
    </filter>
  </svg>

  <nav>
    <div class="indicator" style="--top: {indicatorPosition};"></div>
    {#each routes as route}
      <a
        aria-current={route.path === currentRoute ? "page" : undefined}
        class:icon={true}
        class={route.name.toLowerCase()}
        id="nav-{route.name.toLowerCase()}"
        title={route.name}
        href={dummyLinks ? "#" : route.path}
        on:click={routeChangeSimulator(route)}
      >
        <svelte:component this={route.icon} />
      </a>
    {/each}
  </nav>
</header>

<style>
  header {
    --animation-duration: calc(2 * var(--standard-duration));
    --icons-top-offset: calc(2 * var(--corner-roundness));
    --sidebar-left-padding: 0.5rem;
    --sidebar-icon-size: calc(
      var(--sidebar-width) - var(--sidebar-left-padding)
    );
    padding-top: var(--icons-top-offset);
    padding-left: var(--sidebar-left-padding);
    float: left;
    clip-path: inset(0 0 0 0);
    height: 100vh;
    box-sizing: border-box;
    position: absolute;
    top: 0;
    left: 0;
  }

  nav {
    position: relative;
  }

  .icon,
  .indicator {
    width: var(--sidebar-icon-size);
    height: var(--sidebar-icon-size);
  }

  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon > :global(:only-child) {
    font-size: calc(0.5 * var(--sidebar-icon-size));
    color: #aaa;
    filter: url(#inset-shadow);
    z-index: 2;
    transition: color var(--animation-duration) ease-out;
  }

  .icon[aria-current="page"] > :global(:only-child) {
    color: #1a75ff;
    filter: url(#inset-shadow-selected);
  }

  .icon[aria-current="page"].credits > :global(:only-child) {
    color: #ff1a40;
    filter: url(#inset-shadow);
  }

  .indicator {
    border-top-left-radius: var(--corner-roundness);
    border-bottom-left-radius: var(--corner-roundness);
    position: absolute;
    top: 0;
    background-color: var(--color-offwhite);
    box-shadow: 0 var(--shadow-offset) var(--shadow-blur) 0 #ccc;
    z-index: 1;
    transform: translateY(var(--top));
    transition: transform var(--animation-duration) ease-out;
  }

  .indicator::before,
  .indicator::after {
    content: "";
    height: calc(2 * var(--corner-roundness));
    width: var(--corner-roundness);
    position: absolute;
    right: 0;
  }

  .indicator::before {
    bottom: var(--sidebar-icon-size);
    border-radius: 0 0 var(--corner-roundness) 0;
    box-shadow: 0 var(--corner-roundness) 0 0 var(--color-offwhite);
  }

  .indicator::after {
    top: var(--sidebar-icon-size);
    border-radius: 0 var(--corner-roundness) 0 0;
    box-shadow: 0 calc(-1 * var(--corner-roundness)) 0 0 var(--color-offwhite);
  }
</style>
