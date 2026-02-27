import { useQuery } from '@tanstack/react-query';
import { api } from '../api/client';

export function useInterface() {
  return useQuery({
    queryKey: ['interface'],
    queryFn: () => api.interface.get(),
  });
}
