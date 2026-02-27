import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '../api/client';
import type { CreateClientRequest, UpdateClientRequest } from '../api/types';

export function useClients() {
  const qc = useQueryClient();

  const query = useQuery({
    queryKey: ['clients'],
    queryFn: () => api.clients.list(),
  });

  const create = useMutation({
    mutationFn: (data: CreateClientRequest) => api.clients.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['clients'] }),
  });

  const update = useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateClientRequest }) =>
      api.clients.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['clients'] }),
  });

  const remove = useMutation({
    mutationFn: (id: string) => api.clients.remove(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['clients'] }),
  });

  const enable = useMutation({
    mutationFn: (id: string) => api.clients.enable(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['clients'] }),
  });

  const disable = useMutation({
    mutationFn: (id: string) => api.clients.disable(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['clients'] }),
  });

  return { ...query, create, update, remove, enable, disable };
}
