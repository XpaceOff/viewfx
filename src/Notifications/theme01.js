import { toast } from '@zerodevx/svelte-toast'

export const notification_success = m => toast.push(m, {
    pausable: true,
    theme: {
      '--toastBackground': 'rgba(74, 222, 128, .6)', //#EF4444
      '--toastColor': '#FEF2F2',
      '--toastBarBackground': 'rgba(134, 239, 172, .5)',
      '--toastBarHeight': '2px',
      '--toastBorderRadius': '0.25rem'
    }
})

export const notification_warning = m => toast.push(m, {
    pausable: true,
    theme: {
      '--toastBackground': 'rgba(251, 191, 36, .6)', //#EF4444
      '--toastColor': '#FEF2F2',
      '--toastBarBackground': 'rgba(252, 211, 77, .5)',
      '--toastBarHeight': '2px',
      '--toastBorderRadius': '0.25rem'
    }
})

export const notification_error = m => toast.push(m, {
    pausable: true,
    theme: {
      '--toastBackground': 'rgba(239, 68, 68, 0.6)', //#EF4444
      '--toastColor': '#FEF2F2',
      '--toastBarBackground': 'rgba(220, 38, 38, .5)',
      '--toastBarHeight': '2px',
      '--toastBorderRadius': '0.25rem'
    }
})
