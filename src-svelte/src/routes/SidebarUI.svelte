<script lang="ts">
  import IconSettings from "~icons/ion/settings";
  import IconChat from "~icons/ph/chat-dots-fill";
  import IconDashboard from "~icons/material-symbols/monitor-heart";
  import IconHeartFill from "~icons/ph/heart-fill";
  import IconDatabase from "~icons/fa6-solid/database";
  import { playSoundEffect } from "$lib/sound";
  import { animationSpeed, rootEm } from "$lib/preferences";
  import { onMount } from "svelte";

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
      name: "API Calls",
      path: "/api-calls",
      icon: IconDatabase,
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

  const REGULAR_TRANSITION_DURATION = "calc(2 * var(--standard-duration))";
  export let currentRoute: string;
  export let dummyLinks = false;
  let indicatorPosition: number;
  let transitionDuration = REGULAR_TRANSITION_DURATION;
  let previousRoute = currentRoute;
  let iconLinks = routes.reduce(
    (acc, route) => {
      acc[route.name] = route.path;
      return acc;
    },
    {} as Record<string, string>,
  );

  function routeMatches(sidebarRoute: string, pageRoute: string) {
    if (sidebarRoute === "/") {
      return pageRoute === sidebarRoute;
    }
    return pageRoute.includes(sidebarRoute);
  }

  function getMatchingRoute(newRoute: string) {
    const routeIndex = routes.findIndex((r) => routeMatches(r.path, newRoute));
    return routes[routeIndex];
  }

  function getIndicatorPosition(matchingTab: App.Route) {
    const routeName = matchingTab.name.toLowerCase();
    const routeElementId = `nav-${routeName}`;
    const iconElement = document.getElementById(routeElementId);
    if (!iconElement) {
      // possible before element has mounted
      return 0;
    }
    indicatorPosition = iconElement.offsetTop;
    return indicatorPosition;
  }

  function updateIndicator(newRoute: string) {
    const previousTab = getMatchingRoute(previousRoute);
    const currentTab = getMatchingRoute(newRoute);
    indicatorPosition = getIndicatorPosition(currentTab);
    if (previousTab.name !== currentTab.name) {
      // click on the previous icon will take the user back to that same page
      iconLinks[previousTab.name] = previousRoute;
      // clicking on the new icon will reset to the default page for this icon
      iconLinks[currentTab.name] = currentTab.path;
    }
    previousRoute = newRoute;
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

  onMount(() => {
    const updateIndicator = () => {
      transitionDuration = "0";
      indicatorPosition = getIndicatorPosition(getMatchingRoute(currentRoute));
      setTimeout(() => {
        transitionDuration = REGULAR_TRANSITION_DURATION;
      }, 10);
    };
    const rootEmUnsubscribe = rootEm.subscribe(() => {
      // update 100ms later in case browser takes time to update
      setTimeout(updateIndicator, 100);
    });
    window.addEventListener("resize", updateIndicator);

    return () => {
      rootEmUnsubscribe();
      window.removeEventListener("resize", updateIndicator);
    };
  });

  $: updateIndicator(currentRoute);
</script>

<header style={`--animation-duration: ${transitionDuration};`}>
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
    <div class="indicator" style="--top: {indicatorPosition}px;"></div>
    {#each routes.slice(0, routes.length - 2) as route (route.path)}
      <a
        aria-current={routeMatches(route.path, currentRoute)
          ? "page"
          : undefined}
        class:icon={true}
        class={route.name.toLowerCase()}
        id="nav-{route.name.toLowerCase()}"
        title={route.name}
        href={iconLinks[route.name]}
        on:click={routeChangeSimulator(route)}
      >
        <svelte:component this={route.icon} />
      </a>
    {/each}
    <div class="spacer"></div>
    {#each routes.slice(routes.length - 2, routes.length) as route (route.path)}
      <a
        aria-current={routeMatches(route.path, currentRoute)
          ? "page"
          : undefined}
        class:icon={true}
        class={route.name.toLowerCase()}
        id="nav-{route.name.toLowerCase()}"
        title={route.name}
        href={iconLinks[route.name]}
        on:click={routeChangeSimulator(route)}
      >
        <svelte:component this={route.icon} />
      </a>
    {/each}
  </nav>
</header>

<style>
  header {
    --icons-top-offset: calc(2 * var(--corner-roundness));
    --sidebar-left-padding: 0.5rem;
    --sidebar-icon-size: calc(
      var(--sidebar-width) - var(--sidebar-left-padding)
    );
    padding: var(--icons-top-offset) 0 var(--icons-top-offset)
      var(--sidebar-left-padding);
    float: left;
    clip-path: inset(0 0 0 0);
    height: 100vh;
    box-sizing: border-box;
    overflow: hidden;
    position: absolute;
    top: 0;
    left: 0;
  }

  nav {
    position: relative;
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  nav .spacer {
    flex: 1;
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
