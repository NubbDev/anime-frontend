<script lang="ts">
    import FilledHouse from '$lib/assets/icons/house-blank-filled.svg';
    import LinedHouse from '$lib/assets/icons/house-blank.svg';
    import FilledCalendar from '$lib/assets/icons/calendar-solid.svg';
    import LinedCalendar from '$lib/assets/icons/calendar-thin.svg';
    import FilledDownload from '$lib/assets/icons/inbox-in-filled.svg';
    import LinedDownload from '$lib/assets/icons/inbox-in.svg';
    import FilledProfile from '$lib/assets/icons/circle-user-filled.svg';
    import LineProfile from '$lib/assets/icons/circle-user.svg';
  import { onMount } from 'svelte';
    // assets/icons/house-blank.svg
    const navOptions: { name: string, href: string, selected: string, unSelected: string }[] = [
        {
            name: 'Home',
            href: '/',
            selected: FilledHouse,
            unSelected: LinedHouse
        },
        {
            name: 'Releases',
            href: '/releases',
            selected: FilledCalendar,
            unSelected: LinedCalendar
            
        },
        {
            name: 'Downloads',
            href: '/downloads',
            selected: FilledDownload,
            unSelected: LinedDownload
        },
        {
            name: 'Profile',
            href: '/profile',
            selected: FilledProfile,
            unSelected: LineProfile
        }
    ]

    let path: string;
    onMount(() => {
        path = '/' + window.location.pathname.split('/')[1];
    })
</script>

<nav>
    
    {#each navOptions as option}
        <a href={option.href}>
            {option.selected}
            {#if path === option.href} 
                <img src={option.selected} alt="/" style="fill: white;">
                <span>{option.name}</span>
            {:else}
                <img src={option.unSelected} alt={option.name} >
                <span style="color: var(--icon-unselected);">{option.name}</span>
            {/if}
        </a>
    {/each}
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