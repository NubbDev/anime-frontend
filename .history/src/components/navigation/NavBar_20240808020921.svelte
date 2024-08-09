<script lang="ts">
    import FilledHouse from '$lib/assets/icons/house-blank-filled.svg?component';
    import LinedHouse from '$lib/assets/icons/house-blank.svg?component';
    import FilledCalendar from '$lib/assets/icons/calendar-solid.svg?component';
    import LinedCalendar from '$lib/assets/icons/calendar-thin.svg?component';
    import FilledDownload from '$lib/assets/icons/inbox-in-filled.svg?component';
    import LinedDownload from '$lib/assets/icons/inbox-in.svg?component';
    import FilledProfile from '$lib/assets/icons/circle-user-filled.svg?component';
    import LineProfile from '$lib/assets/icons/circle-user.svg?component';
  import { onMount, SvelteComponent, type ComponentType } from 'svelte';
  import type { SVGAttributes } from 'svelte/elements';
    // assets/icons/house-blank.svg
    const navOptions: { name: string, href: string, selected: ComponentType<SvelteComponent<SVGAttributes<SVGSVGElement>, any, any>>, unSelected: ComponentType<SvelteComponent<SVGAttributes<SVGSVGElement>, any, any>> }[] = [
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
            {#if path === option.href} 
                <!-- <img src={option.selected} alt="/" style="fill: white;">
                  -->
                    <svelte:component this={option.selected} fill="white" />
                    <span>{option.name}</span>
                {:else}
                    <svelte:component this={option.selected} fill="white" />
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