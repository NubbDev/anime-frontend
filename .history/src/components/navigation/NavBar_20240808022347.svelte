<script lang="ts">
    import FilledHouse from '$lib/assets/icons/house-blank-filled.svg?component';
    import LinedHouse from '$lib/assets/icons/house-blank.svg?component';
    import FilledCalendar from '$lib/assets/icons/calendar-solid.svg?component';
    import LinedCalendar from '$lib/assets/icons/calendar-thin.svg?component';
    import FilledDownload from '$lib/assets/icons/inbox-in-filled.svg?component';
    import LinedDownload from '$lib/assets/icons/inbox-in.svg?component';
    import FilledProfile from '$lib/assets/icons/circle-user-filled.svg?component';
    import LineProfile from '$lib/assets/icons/circle-user.svg?component';
    import { onMount } from 'svelte';
    let path: string;
    onMount(async () => {
        path = '/' + window.location.pathname.split('/')[1];
    })
</script>

<nav>
    <a href="/">
        {#if path === '/'}
            <FilledHouse />
        {:else}
            <LinedHouse />
        {/if}
        <span>Home</span>
    </a>
    <a href="/">
        {#if path === '/releases'}
            <FilledCalendar />
        {:else}
            <LinedCalendar />
        {/if}
        <span>Releases</span>
    </a>
    <a href="/">
        {#if path === '/downloads'}
            <FilledDownload />
        {:else}
            <LinedDownload />
        {/if}
        <span>Downloads</span>
    </a>
    <a href="/">
        {#if path === '/profile'}
            <FilledProfile />
        {:else}
            <LineProfile />
        {/if}
        <span>Profile</span>
    </a>
</nav>

<style>
    nav {
        display: flex;
        justify-content: space-between;
        box-sizing: border-box;
        padding: 0 10vw;
        position: fixed;
        bottom: 0;
        left: 0;
        margin: 0;
        width: 100vw;
        background-color: rgba(18, 18, 18, 0.8);
        backdrop-filter: 16;
    }

    nav a img {
        width: 35px;
        height: 35px;
    }
    nav a span {
        color: white;
        font-size: 0.6rem;
        text-decoration: none;
    }
    nav a {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-decoration: none;
    }
</style>